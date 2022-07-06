/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

mod component_replication;
mod reflect_object;

use bevy::{ecs::entity::EntityMap, prelude::*, reflect::TypeRegistry, utils::HashMap};
use bevy_renet::renet::{RenetClient, RenetServer, ServerEvent};
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    any::{type_name, TypeId},
    time::Duration,
};

use super::{client, Channel};
use component_replication::ComponentReplicationPlugins;
use reflect_object::{ReflectObject, ReflectObjectPlugin};

pub(super) struct UnreliableMessagePlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum NetworkStage {
    Tick,
}

impl Plugin for UnreliableMessagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkTick>()
            .init_resource::<ReceivedServerTick>()
            .init_resource::<NetworkEntityMap>()
            .init_resource::<ClientAcks>()
            .add_plugins(ComponentReplicationPlugins)
            .add_plugin(ReflectObjectPlugin)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                Self::insert_remove_client_acks_system.run_if_resource_exists::<RenetServer>(),
            )
            .add_stage_before(
                CoreStage::Update,
                NetworkStage::Tick,
                FixedTimestepStage::new(Duration::from_secs_f64(Self::TIMESTEP))
                    .with_stage(SystemStage::single(
                        Self::receive_client_message_system.run_if_resource_exists::<RenetServer>(),
                    ))
                    .with_stage(SystemStage::single(
                        Self::send_server_message_system.run_if_resource_exists::<RenetServer>(),
                    ))
                    .with_stage(SystemStage::single(
                        iyes_loopless::condition::IntoConditionalExclusiveSystem::run_if(
                            Self::receive_server_message_system,
                            client::connected,
                        )
                        .at_start(),
                    ))
                    .with_stage(SystemStage::single(
                        Self::send_client_message_system.run_if(client::connected),
                    )),
            )
            .add_system(Self::server_reset_system.run_if_resource_removed::<RenetServer>())
            .add_system(Self::client_reset_system.run_if_resource_removed::<RenetClient>());
    }
}

impl UnreliableMessagePlugin {
    const TIMESTEP: f64 = 0.1;

    fn insert_remove_client_acks_system(
        mut server_events: EventReader<ServerEvent>,
        client_acks: Option<ResMut<ClientAcks>>,
    ) {
        if let Some(mut client_acks) = client_acks {
            for event in server_events.iter() {
                match event {
                    ServerEvent::ClientConnected(id, _) => {
                        client_acks.insert(*id, 0);
                    }
                    ServerEvent::ClientDisconnected(id) => {
                        client_acks.remove(id);
                    }
                }
            }
        }
    }

    fn receive_client_message_system(
        mut client_acks: ResMut<ClientAcks>,
        mut server: ResMut<RenetServer>,
    ) {
        for client_id in server.clients_id() {
            let mut messages = Vec::<ClientUnreliableMessage>::new();
            while let Some(message) = server.receive_message(client_id, Channel::Unreliable.id()) {
                match rmp_serde::from_slice(&message) {
                    Ok(message) => messages.push(message),
                    Err(error) => {
                        error!(
                            "Unable to deserialize unreliable message from client {}: {}",
                            client_id, error
                        );
                        continue;
                    }
                };
            }

            if let Some(last_message) = messages.iter().max_by_key(|message| message.tick_ack) {
                let last_tick_ack = client_acks.entry(client_id).or_default();
                if *last_tick_ack < last_message.tick_ack {
                    *last_tick_ack = last_message.tick_ack;
                }
            }
        }
    }

    fn send_server_message_system(
        mut set: ParamSet<(&World, ResMut<NetworkTick>, ResMut<RenetServer>)>,
        client_acks: Res<ClientAcks>,
        type_registry: Res<TypeRegistry>,
        replicating_entities: Query<(Entity, &Replication)>,
    ) {
        set.p1().0 += 1;
        let network_tick = set.p1().0;

        let mut messages = HashMap::new();
        let world = set.p0();
        let type_registry = type_registry.read();
        for (entity, replication) in replicating_entities.iter() {
            for (&client_id, &tick_ack) in client_acks.iter() {
                let message = messages
                    .entry(client_id)
                    .or_insert(ServerUnreliableMessage::new(network_tick));
                let changes = message.component_changes.entry(entity).or_default();
                for (&type_id, tick_changes) in replication.iter() {
                    if tick_changes.changed < tick_ack && tick_changes.removed < tick_ack {
                        // Change is already acknowledged
                        continue;
                    }

                    let registration = type_registry
                        .get(type_id)
                        .expect("Unable to get registration for replicated component");

                    if tick_changes.removed > tick_changes.changed {
                        changes.push(Change::Removed(registration.name().to_string()));
                    } else if tick_changes.changed > tick_ack {
                        let reflect_component =
                            registration.data::<ReflectComponent>().unwrap_or_else(|| {
                                panic!(
                                    "Type {} doesn't implement {}",
                                    registration.name(),
                                    type_name::<ReflectComponent>()
                                )
                            });
                        let reflect_object: ReflectObject = reflect_component
                            .reflect_component(world, entity)
                            .unwrap_or_else(|| {
                                panic!("Unable to reflect component {}", registration.name())
                            })
                            .clone_value()
                            .into();
                        changes.push(Change::Changed(reflect_object));
                    }
                }
            }
        }

        let mut server = set.p2();
        for (client_id, message) in messages {
            let message = rmp_serde::to_vec(&message).unwrap_or_else(|error| {
                panic!("Unable to serialize unreliable server message: {}", error)
            });
            server.send_message(client_id, Channel::Unreliable.id(), message);
        }
    }

    fn receive_server_message_system(world: &mut World) {
        let mut messages = Vec::<ServerUnreliableMessage>::new();
        let mut client = world.resource_mut::<RenetClient>();
        while let Some(message) = client.receive_message(Channel::Unreliable.id()) {
            match rmp_serde::from_slice(&message) {
                Ok(message) => messages.push(message),
                Err(error) => {
                    error!("Unable to deserialize unreliable message: {}", error);
                    continue;
                }
            };
        }

        let mut received_server_tick = world.resource_mut::<ReceivedServerTick>();
        let last_message = match messages.iter().max_by_key(|message| message.tick) {
            Some(last_message) if received_server_tick.0 < last_message.tick => last_message,
            _ => return,
        };
        received_server_tick.0 = last_message.tick;

        // Temorary take resources to avoid borrowing issues
        let type_registry = world.remove_resource::<TypeRegistry>().unwrap();
        let mut entity_map = world.remove_resource::<NetworkEntityMap>().unwrap();

        let read_registry = type_registry.read();
        for (&server_entity, changes) in last_message.component_changes.iter() {
            let local_entity = *entity_map
                .entry(server_entity)
                .or_insert_with(|| world.spawn().id());

            for change in changes.iter() {
                let type_name = match change {
                    Change::Changed(reflect_object) => reflect_object.type_name(),
                    Change::Removed(type_name) => type_name,
                };

                let registration = match read_registry.get_with_name(type_name) {
                    Some(registration) => registration,
                    None => {
                        error!("Unable to get registration for type {}", type_name);
                        continue;
                    }
                };

                let reflect_component = match registration.data::<ReflectComponent>() {
                    Some(reflect_component) => reflect_component,
                    None => {
                        error!("Unable to reflect component for type {}", type_name);
                        continue;
                    }
                };

                match change {
                    Change::Changed(reflect_object) => {
                        // TODO 0.8: Use apply_or_insert
                        if world
                            .entity(local_entity)
                            .contains_type_id(registration.type_id())
                        {
                            reflect_component.apply_component(
                                world,
                                local_entity,
                                &***reflect_object,
                            );
                        } else {
                            reflect_component.add_component(
                                world,
                                local_entity,
                                &***reflect_object,
                            );
                        }
                    }
                    Change::Removed(_) => reflect_component.remove_component(world, local_entity),
                }
            }
        }
        drop(read_registry);

        world.insert_resource(type_registry);
        world.insert_resource(entity_map);
    }

    fn send_client_message_system(
        received_server_tick: Res<ReceivedServerTick>,
        mut network_tick: ResMut<NetworkTick>,
        mut client: ResMut<RenetClient>,
    ) {
        network_tick.0 += 1;

        let message = rmp_serde::to_vec(&ClientUnreliableMessage {
            tick_ack: received_server_tick.0,
        })
        .unwrap_or_else(|error| panic!("Unable to serialize unreliable client message: {}", error));
        client.send_message(Channel::Unreliable.id(), message);
    }

    fn server_reset_system(mut commands: Commands) {
        commands.insert_resource(NetworkTick::default());
        commands.insert_resource(ClientAcks::default());
    }

    fn client_reset_system(mut commands: Commands) {
        commands.insert_resource(NetworkTick::default());
        commands.insert_resource(ReceivedServerTick::default());
        commands.insert_resource(NetworkEntityMap::default());
    }
}

/// Current network tick
/// Used on server and clients
struct NetworkTick(u32);

impl Default for NetworkTick {
    fn default() -> Self {
        Self(1) // Start with 1 to mark all initial values as non-acknowledged
    }
}

/// Last received tick from server
/// Used only on clients
#[derive(Default)]
struct ReceivedServerTick(u32);

/// Last acknowledged server ticks from all clients
/// Used only on server
#[derive(Default, Deref, DerefMut)]
struct ClientAcks(HashMap<u64, u32>);

/// Changed world data and current tick from server
#[derive(Serialize, Deserialize)]
struct ServerUnreliableMessage {
    tick: u32,
    component_changes: HashMap<Entity, Vec<Change>>,
}

impl ServerUnreliableMessage {
    fn new(tick: u32) -> Self {
        Self {
            tick,
            component_changes: Default::default(),
        }
    }
}

/// Type of component or resource change.
#[derive(Serialize, Deserialize)]
enum Change {
    Changed(ReflectObject),
    Removed(String),
}

/// Input and last received server tick from client
#[derive(Serialize, Deserialize)]
struct ClientUnreliableMessage {
    tick_ack: u32,
}

/// Maps server entities to client entities.
/// Used only on client.
#[derive(Default, Deref, DerefMut)]
struct NetworkEntityMap(EntityMap);

/// Contains information about changes on network ticks for all replicated [`TypeId`]
/// of the entity (when used as a component) and all resources (when used as a resource).
/// This information is used by the server to decide what data to include in packets for clients.
#[derive(Component, Default, Deref, DerefMut)]
struct Replication(HashMap<TypeId, ChangeTicks>);

/// Network ticks with resource or component changes.
#[derive(Clone, Copy, Default)]
struct ChangeTicks {
    changed: u32,
    removed: u32,
}

#[cfg(test)]
mod tests {
    use crate::core::network::tests::{NetworkPreset, TestNetworkPlugin};

    use super::*;

    #[test]
    fn client_acks_insert_and_remove() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        let mut client = app.world.resource_mut::<RenetClient>();
        client.disconnect();
        let client_id = client.client_id();

        let client_acks = app.world.resource::<ClientAcks>();
        assert!(client_acks.contains_key(&client_id));

        app.update();
        app.update();

        let client_acks = app.world.resource::<ClientAcks>();
        assert!(!client_acks.contains_key(&client_id));
    }

    #[test]
    fn spawned_entity_replicates() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        let server_entity = app.world.spawn().insert(Replication::default()).id();

        wait_for_network_tick(&mut app);

        let network_tick = app.world.resource::<NetworkTick>();
        assert_eq!(network_tick.0, NetworkTick::default().0 + 2, "Network tick should be increased by two since int test client and server in the same world");

        // Remove server entity before client replicates it (since in test client and server in the same world)
        app.world.entity_mut(server_entity).despawn();

        wait_for_network_tick(&mut app);

        let client_entity = app
            .world
            .query::<Entity>()
            .iter(&app.world)
            .next()
            .expect("Server entity should be replicated to the client"); // TODO 0.8: Use single

        let entity_map = app.world.resource::<NetworkEntityMap>();
        let mapped_entity = entity_map
            .get(server_entity)
            .expect("Server entity should be mapped on client");
        assert_eq!(
            mapped_entity, client_entity,
            "Mapped entity should correspond to the replicated entity on client"
        );

        wait_for_network_tick(&mut app);

        let client_acks = app.world.resource::<ClientAcks>();
        let client = app.world.resource::<RenetClient>();
        let tick_ack = *client_acks
            .get(&client.client_id())
            .expect("The client ack should be received");
        assert_eq!(
            tick_ack,
            NetworkTick::default().0 + 1,
            "Acknowledged tick should be increased by one (since sending happens before the server system increments the tick)"
        );
    }

    #[test]
    fn inserted_component_replicates() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        const TRANSFORM: Transform = Transform::identity();

        let replicated_entity = app
            .world
            .spawn()
            .insert(Replication::default())
            .insert(TRANSFORM)
            .id();

        let mut entity_map = app.world.resource_mut::<NetworkEntityMap>();
        entity_map.insert(replicated_entity, replicated_entity); // Map an entity to itself so that the client thinks it has already been spawned

        wait_for_network_tick(&mut app);

        // Remove transform before client replicates it (since in test client and server in the same world)
        app.world
            .entity_mut(replicated_entity)
            .remove::<Transform>();

        wait_for_network_tick(&mut app);

        let replicated_transform = *app
            .world
            .get::<Transform>(replicated_entity)
            .expect("The client should replicate the transform component");

        assert_eq!(
            replicated_transform, TRANSFORM,
            "The replicated entity transform should match the entity transform on the server."
        );
    }

    #[test]
    fn removed_component_replicates() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        // Mark transform component as removed
        let mut replication = Replication::default();
        replication.insert(
            TypeId::of::<Transform>(),
            ChangeTicks {
                changed: 0,
                removed: 1,
            },
        );

        let replicated_entity = app.world.spawn().insert(replication).id();

        let mut entity_map = app.world.resource_mut::<NetworkEntityMap>();
        entity_map.insert(replicated_entity, replicated_entity); // Map an entity to itself so that the client thinks it has already been spawned

        wait_for_network_tick(&mut app);

        // Insert transform before client replicates its removal (since in test client and server in the same world)
        app.world
            .entity_mut(replicated_entity)
            .insert(Transform::default());

        wait_for_network_tick(&mut app);

        assert!(
            app.world.get::<Transform>(replicated_entity).is_none(),
            "Client should replicate the transform removal"
        );
    }

    #[test]
    fn client_resets() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Client));

        app.update();

        // Modify resources to test reset
        app.world.resource_mut::<NetworkTick>().0 += 1;
        app.world.resource_mut::<ReceivedServerTick>().0 += 1;
        app.world
            .resource_mut::<NetworkEntityMap>()
            .insert(Entity::from_raw(0), Entity::from_raw(0));

        app.world.remove_resource::<RenetClient>();

        app.update();

        assert_eq!(
            app.world.resource::<NetworkTick>().0,
            NetworkTick::default().0,
            "Resource {} should be defaulted",
            type_name::<NetworkTick>()
        );
        assert_eq!(
            app.world.resource::<ReceivedServerTick>().0,
            ReceivedServerTick::default().0,
            "Resource {} should be defaulted",
            type_name::<ReceivedServerTick>()
        );
        assert_eq!(
            app.world.resource::<NetworkEntityMap>().keys().count(),
            0,
            "Resource {} should be empty",
            type_name::<NetworkEntityMap>()
        );
    }

    #[test]
    fn server_resets() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Server));

        app.update();

        // Modify resources to test reset
        app.world.resource_mut::<NetworkTick>().0 += 1;
        app.world.resource_mut::<ClientAcks>().insert(0, 0);

        app.world.remove_resource::<RenetServer>();

        app.update();

        assert_eq!(
            app.world.resource::<NetworkTick>().0,
            NetworkTick::default().0,
            "Resource {} should be defaulted",
            type_name::<NetworkTick>()
        );
        assert_eq!(
            app.world.resource::<ClientAcks>().len(),
            0,
            "Resource {} should be empty",
            type_name::<NetworkEntityMap>()
        );
    }

    // TODO 0.8: Use [`Time::update_with_instant`]
    fn wait_for_network_tick(app: &mut App) {
        let init_time = app.world.resource::<Time>().seconds_since_startup();
        app.update();
        while app.world.resource::<Time>().seconds_since_startup() - init_time
            < UnreliableMessagePlugin::TIMESTEP
        {
            app.update();
        }
    }
}

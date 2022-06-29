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

mod reflect_object;

use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::{RenetClient, RenetServer};
use iyes_loopless::prelude::*;
use reflect_object::ReflectObjectPlugin;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{client, Channel};

pub(super) struct UnreliableMessagePlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum NetworkStage {
    Tick,
}

impl Plugin for UnreliableMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ReflectObjectPlugin)
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
                        Self::receive_server_message_system.run_if(client::connected),
                    ))
                    .with_stage(SystemStage::single(
                        Self::send_client_message_system.run_if(client::connected),
                    )),
            )
            .add_system(Self::server_ticks_init_system.run_if_resource_added::<RenetServer>())
            .add_system(Self::client_ticks_init_system.run_if_resource_added::<RenetClient>())
            .add_system(Self::server_ticks_remove_system.run_if_resource_removed::<RenetServer>())
            .add_system(Self::client_ticks_remove_system.run_if_resource_removed::<RenetClient>());
    }
}

impl UnreliableMessagePlugin {
    const TIMESTEP: f64 = 0.1;

    fn receive_client_message_system(
        mut received_client_ticks: ResMut<ReceivedClientTicks>,
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

            if let Some(last_message) = messages.iter().max_by_key(|x| x.tick) {
                let last_tick = received_client_ticks.entry(client_id).or_default();
                if *last_tick < last_message.tick {
                    *last_tick = last_message.tick;
                }
            }
        }
    }

    fn send_server_message_system(mut tick: ResMut<Tick>, mut server: ResMut<RenetServer>) {
        tick.0 += 1;

        match rmp_serde::to_vec(&ServerUnreliableMessage { tick: tick.0 }) {
            Ok(message) => server.broadcast_message(Channel::Unreliable.id(), message),
            Err(error) => error!("Unable to serialize unreliable server message: {}", error),
        };
    }

    fn receive_server_message_system(
        mut received_server_tick: ResMut<ReceivedServerTick>,
        mut client: ResMut<RenetClient>,
    ) {
        let mut messages = Vec::<ServerUnreliableMessage>::new();
        while let Some(message) = client.receive_message(Channel::Unreliable.id()) {
            match rmp_serde::from_slice(&message) {
                Ok(message) => messages.push(message),
                Err(error) => {
                    error!("Unable to deserialize unreliable message: {}", error);
                    continue;
                }
            };
        }

        if let Some(last_message) = messages.iter().max_by_key(|x| x.tick) {
            if received_server_tick.0 < last_message.tick {
                received_server_tick.0 = last_message.tick;
            }
        }
    }

    fn send_client_message_system(
        received_server_tick: Res<ReceivedServerTick>,
        mut tick: ResMut<Tick>,
        mut client: ResMut<RenetClient>,
    ) {
        tick.0 += 1;

        match rmp_serde::to_vec(&ClientUnreliableMessage {
            tick: received_server_tick.0,
        }) {
            Ok(message) => client.send_message(Channel::Unreliable.id(), message),
            Err(error) => error!("Unable to serialize unreliable client message: {}", error),
        };
    }

    fn server_ticks_init_system(mut commands: Commands) {
        commands.init_resource::<Tick>();
        commands.init_resource::<ReceivedClientTicks>();
    }

    fn client_ticks_init_system(mut commands: Commands) {
        commands.init_resource::<Tick>();
        commands.init_resource::<ReceivedServerTick>();
    }

    fn server_ticks_remove_system(mut commands: Commands) {
        commands.remove_resource::<Tick>();
        commands.remove_resource::<ReceivedClientTicks>();
    }

    fn client_ticks_remove_system(mut commands: Commands) {
        commands.remove_resource::<Tick>();
        commands.remove_resource::<ReceivedServerTick>();
    }
}

/// Current network tick
/// Available on server and clients
#[derive(Default)]
struct Tick(u64);

/// Last received tick from server
/// Only available on clients
#[derive(Default)]
struct ReceivedServerTick(u64);

/// Last received ticks from all clients
/// Only available on server
#[derive(Default, Deref, DerefMut)]
struct ReceivedClientTicks(HashMap<u64, u64>);

/// Changed world data and current tick from server
#[derive(Serialize, Deserialize)]
struct ServerUnreliableMessage {
    tick: u64,
}

/// Input and last received server tick from client
#[derive(Serialize, Deserialize)]
struct ClientUnreliableMessage {
    tick: u64,
}

#[cfg(test)]
mod tests {
    use crate::core::network::tests::{NetworkPreset, TestNetworkPlugin};

    use super::*;

    #[test]
    fn client_ticks_init_and_cleanup() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Client));

        app.update();

        assert!(
            app.world.contains_resource::<Tick>(),
            "The tick resource should be created when connected"
        );
        assert!(
            app.world.contains_resource::<ReceivedServerTick>(),
            "The received server tick resource should be created when connected"
        );

        app.world.remove_resource::<RenetClient>();

        app.update();

        assert!(
            !app.world.contains_resource::<Tick>(),
            "The tick resource should be removed when disconnected"
        );
        assert!(
            !app.world.contains_resource::<ReceivedServerTick>(),
            "The received server tick resource should be removed when disconnected"
        );
    }

    #[test]
    fn server_ticks_init_and_cleanup() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Server));

        app.update();

        assert!(
            app.world.contains_resource::<Tick>(),
            "The tick resource should be created on server creation"
        );
        assert!(
            app.world.contains_resource::<ReceivedClientTicks>(),
            "The received client ticks resource should be created on server creation"
        );

        app.world.remove_resource::<RenetServer>();

        app.update();

        assert!(
            !app.world.contains_resource::<Tick>(),
            "The tick resource should be removed on server shutdown"
        );
        assert!(
            !app.world.contains_resource::<ReceivedClientTicks>(),
            "The received client ticks resource should be removed on server shutdown"
        );
    }

    #[test]
    fn sending_and_receiving() {
        let mut app = App::new();
        app.add_plugin(UnreliableMessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        // TODO 0.8: Use [`Time::update_with_instant`]
        let init_time = app.world.resource::<Time>().seconds_since_startup();
        app.update();
        while app.world.resource::<Time>().seconds_since_startup() - init_time
            < UnreliableMessagePlugin::TIMESTEP
        {
            app.update();
        }

        let tick = app.world.resource::<Tick>();
        assert_eq!(tick.0, 2, "Server tick should be increased by two since we have client and server in the same world");

        // TODO: Test if the client tick was acknowledged
        // after resolving https://github.com/lucaspoffo/renet/pull/17
    }
}

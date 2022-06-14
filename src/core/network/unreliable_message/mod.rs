/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation; either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a get of the GNU Affero General Public License
 *  along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 */

use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_renet::renet::{RenetClient, RenetServer};
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Channel, NetworkingState};

pub(super) struct UnreliableMessagePlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
enum NetworkStage {
    Tick,
}

impl Plugin for UnreliableMessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            NetworkStage::Tick,
            FixedTimestepStage::new(Duration::from_secs_f64(Self::TIMESTEP))
                .with_stage(SystemStage::single(
                    Self::receive_client_message_system.run_in_state(NetworkingState::Hosting),
                ))
                .with_stage(SystemStage::single(
                    Self::send_server_message_system.run_in_state(NetworkingState::Hosting),
                ))
                .with_stage(SystemStage::single(
                    Self::receive_server_message_system.run_in_state(NetworkingState::Connected),
                ))
                .with_stage(SystemStage::single(
                    Self::send_client_message_system.run_in_state(NetworkingState::Connected),
                )),
        )
        .add_enter_system(NetworkingState::Hosting, Self::server_ticks_init_system)
        .add_enter_system(NetworkingState::Connected, Self::client_ticks_init_system)
        .add_exit_system(NetworkingState::Hosting, Self::server_ticks_reset_system)
        .add_exit_system(NetworkingState::Connected, Self::client_ticks_reset_system);
    }
}

impl UnreliableMessagePlugin {
    const TIMESTEP: f64 = 0.1;

    fn receive_client_message_system(
        mut client_acks: ResMut<ClientAcks>,
        mut server: ResMut<RenetServer>,
    ) {
        for client_id in server.clients_id() {
            let mut messages = Vec::<ClientUnreliableMessage>::new();
            while let Some(message) = server.receive_message(client_id, Channel::Unreliable.id()) {
                match bincode::deserialize(&message) {
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
                let last_tick = client_acks.entry(client_id).or_default();
                if *last_tick < last_message.tick {
                    *last_tick = last_message.tick;
                }
            }
        }
    }

    fn send_server_message_system(
        mut server_tick: ResMut<ServerTick>,
        mut server: ResMut<RenetServer>,
    ) {
        server_tick.0 += 1;

        match bincode::serialize(&ServerUnreliableMessage {
            tick: server_tick.0,
        }) {
            Ok(message) => server.broadcast_message(Channel::Unreliable.id(), message),
            Err(error) => error!("Unable to serialize unreliable server message: {}", error),
        };
    }

    fn receive_server_message_system(
        mut server_tick: ResMut<ServerTick>,
        mut client: ResMut<RenetClient>,
    ) {
        let mut messages = Vec::<ServerUnreliableMessage>::new();
        while let Some(message) = client.receive_message(Channel::Unreliable.id()) {
            match bincode::deserialize(&message) {
                Ok(message) => messages.push(message),
                Err(error) => {
                    error!("Unable to deserialize unreliable message: {}", error);
                    continue;
                }
            };
        }

        if let Some(last_message) = messages.iter().max_by_key(|x| x.tick) {
            if server_tick.0 < last_message.tick {
                server_tick.0 = last_message.tick;
            }
        }
    }

    fn send_client_message_system(
        server_tick: Res<ServerTick>,
        mut client_tick: ResMut<ClientTick>,
        mut client: ResMut<RenetClient>,
    ) {
        client_tick.0 += 1;

        match bincode::serialize(&ClientUnreliableMessage {
            tick: server_tick.0,
        }) {
            Ok(message) => client.send_message(Channel::Unreliable.id(), message),
            Err(error) => error!("Unable to serialize unreliable client message: {}", error),
        };
    }

    fn server_ticks_init_system(mut commands: Commands) {
        commands.init_resource::<ServerTick>();
        commands.init_resource::<ClientAcks>();
    }

    fn client_ticks_init_system(mut commands: Commands) {
        commands.init_resource::<ServerTick>();
        commands.init_resource::<ClientTick>();
    }

    fn server_ticks_reset_system(mut commands: Commands) {
        commands.remove_resource::<ServerTick>();
        commands.remove_resource::<ClientAcks>();
    }

    fn client_ticks_reset_system(mut commands: Commands) {
        commands.remove_resource::<ServerTick>();
        commands.remove_resource::<ClientTick>();
    }
}

/// Current tick number of the server
/// Available on server and clients
#[derive(Default)]
struct ServerTick(u64);

/// Client's current tick number
/// Only available on clients
#[derive(Default)]
struct ClientTick(u64);

/// Acknowledged ticks from all clients
/// Only available on server
#[derive(Default, Deref, DerefMut)]
struct ClientAcks(HashMap<u64, u64>);

/// World snapshot sent from the server
#[derive(Serialize, Deserialize)]
struct ServerUnreliableMessage {
    tick: u64,
}

/// Client input and ack of the last received snapshot
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
        app.add_plugin(TestUnreliableMessagePlugin::new(
            None,
            NetworkingState::Connected,
        ));

        app.update();

        assert!(
            app.world.contains_resource::<ServerTick>(),
            "The server tick resource should be created when connected"
        );
        assert!(
            app.world.contains_resource::<ClientTick>(),
            "The client tick resource should be created when connected"
        );

        app.world
            .insert_resource(NextState(NetworkingState::NoSocket));

        app.update();

        assert!(
            !app.world.contains_resource::<ServerTick>(),
            "The server tick resource should be removed when disconnected"
        );
        assert!(
            !app.world.contains_resource::<ClientTick>(),
            "The client tick resource should be removed when disconnected"
        );
    }

    #[test]
    fn server_ticks_init_and_cleanup() {
        let mut app = App::new();
        app.add_plugin(TestUnreliableMessagePlugin::new(
            None,
            NetworkingState::Hosting,
        ));

        app.update();

        assert!(
            app.world.contains_resource::<ServerTick>(),
            "The server tick resource should be created on server creation"
        );
        assert!(
            app.world.contains_resource::<ClientAcks>(),
            "The clients acks resource should be created on server creation"
        );

        app.world
            .insert_resource(NextState(NetworkingState::NoSocket));

        app.update();

        assert!(
            !app.world.contains_resource::<ServerTick>(),
            "The server tick resource should be removed on server shutdown"
        );
        assert!(
            !app.world.contains_resource::<ClientAcks>(),
            "The clients acks resource should be removed on server shutdown"
        );
    }

    #[test]
    fn sending_and_receiving() {
        let mut app = App::new();
        app.add_plugin(TestUnreliableMessagePlugin::new(
            Some(NetworkPreset::ServerAndClient { connected: true }),
            NetworkingState::Hosting,
        ));

        // TODO 0.8: Use [`Time::update_with_instant`]
        let init_time = app.world.resource::<Time>().seconds_since_startup();
        app.update();
        while app.world.resource::<Time>().seconds_since_startup() - init_time
            < UnreliableMessagePlugin::TIMESTEP
        {
            app.update();
        }

        assert_eq!(
            app.world.resource::<ServerTick>().0,
            1,
            "Server tick should be increased after timestep"
        );

        app.world
            .insert_resource(NextState(NetworkingState::Connected));

        // Wait for the next timestep since system wasn't executed in [`NetworkingState::Hosting`]
        // state
        // TODO 0.8: Use [`Time::update_with_instant`]
        let init_time = app.world.resource::<Time>().seconds_since_startup();
        app.update();
        while app.world.resource::<Time>().seconds_since_startup() - init_time
            < UnreliableMessagePlugin::TIMESTEP
        {
            app.update();
        }

        assert_eq!(
            app.world.resource::<ClientTick>().0,
            1,
            "Client tick should be increased after timestep"
        );

        // TODO: Test if the client tick was acknowledged
        // after resolving https://github.com/lucaspoffo/renet/pull/17
    }

    struct TestUnreliableMessagePlugin {
        preset: Option<NetworkPreset>,
        networking_state: NetworkingState,
    }

    impl TestUnreliableMessagePlugin {
        fn new(preset: Option<NetworkPreset>, networking_state: NetworkingState) -> Self {
            Self {
                preset,
                networking_state,
            }
        }
    }

    impl Plugin for TestUnreliableMessagePlugin {
        fn build(&self, app: &mut App) {
            if let Some(preset) = self.preset {
                app.add_plugin(TestNetworkPlugin::new(preset));
            }

            app.add_loopless_state(self.networking_state)
                .add_plugin(UnreliableMessagePlugin);
        }
    }
}

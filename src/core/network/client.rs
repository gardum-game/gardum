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

use bevy::prelude::*;
use bevy_renet::renet::{ConnectToken, RenetClient, RenetConnectionConfig};
use clap::Args;
use iyes_loopless::prelude::*;
use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use super::{Channel, NetworkingState, DEFAULT_PORT, PROTOCOL_ID, PUBLIC_GAME_KEY};
use crate::core::cli::{Opts, SubCommand};

pub(super) struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            Self::enter_connecting_system
                .run_in_state(NetworkingState::NoSocket)
                .run_if_resource_added::<RenetClient>(),
        )
        .add_system(
            Self::enter_connected_system
                .run_in_state(NetworkingState::Connecting)
                .run_if(connected),
        )
        .add_exit_system(NetworkingState::Connected, Self::disconnect_system)
        .add_exit_system(
            NetworkingState::Connecting,
            Self::client_removal_system.run_if_not(connected),
        );

        let opts = app
            .world
            .get_resource::<Opts>()
            .expect("Command line options should be initialized before client plugin");
        if let Some(SubCommand::Connect(connectioin_settings)) = &opts.subcommand {
            let settings = connectioin_settings.clone();
            app.insert_resource(settings.create_client().expect("Unable to open connection"));
            app.insert_resource(settings);
        } else {
            app.insert_resource(ConnectionSettings::default());
        }
    }
}

impl ClientPlugin {
    fn enter_connecting_system(mut commands: Commands) {
        commands.insert_resource(NextState(NetworkingState::Connecting));
    }

    fn enter_connected_system(mut commands: Commands) {
        commands.insert_resource(NextState(NetworkingState::Connected));
    }

    fn disconnect_system(mut commands: Commands, mut client: ResMut<RenetClient>) {
        client.disconnect();
        commands.remove_resource::<RenetClient>();
    }

    fn client_removal_system(mut commands: Commands) {
        commands.remove_resource::<RenetClient>();
    }
}

fn connected(client: Res<RenetClient>) -> bool {
    client.is_connected()
}

#[derive(Args, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct ConnectionSettings {
    /// Server IP address.
    #[clap(short, long, default_value_t = ConnectionSettings::default().ip)]
    pub(crate) ip: String,

    /// Server port.
    #[clap(short, long, default_value_t = ConnectionSettings::default().port)]
    pub(crate) port: u16,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: DEFAULT_PORT,
        }
    }
}

impl ConnectionSettings {
    pub(crate) fn create_client(&self) -> Result<RenetClient, Box<dyn Error>> {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let client_id = current_time.as_millis() as u64;
        let ip = self.ip.parse()?;
        let token = ConnectToken::generate(
            current_time,
            PROTOCOL_ID,
            300,
            client_id,
            15,
            vec![SocketAddr::new(ip, self.port)],
            None,
            &PUBLIC_GAME_KEY,
        )?;
        RenetClient::new(
            current_time,
            UdpSocket::bind((ip, 0))?,
            client_id,
            token,
            RenetConnectionConfig {
                channels_config: Channel::config(),
                ..Default::default()
            },
        )
        .map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::network::tests::{NetworkPreset, TestNetworkPlugin};

    #[test]
    fn defaulted_without_connect() {
        let mut app = App::new();
        app.init_resource::<Opts>();
        app.add_plugin(TestClientPlugin::new(NetworkingState::NoSocket));

        assert_eq!(
            *app.world.resource::<ConnectionSettings>(),
            ConnectionSettings::default(),
            "Connection settings should be initialized with defaults without host command"
        );
        assert!(
            app.world.get_resource::<RenetClient>().is_none(),
            "Connection should't be opened"
        );
    }

    #[test]
    fn initializes_from_connect() {
        let mut app = App::new();
        let connection_settings = ConnectionSettings {
            port: 0,
            ..Default::default()
        };
        app.world.insert_resource(Opts {
            subcommand: Some(SubCommand::Connect(connection_settings.clone())),
        });
        app.add_plugin(TestClientPlugin::new(NetworkingState::NoSocket));

        assert_eq!(
            *app.world.resource::<ConnectionSettings>(),
            connection_settings,
            "Connection settings should be initialized with parameters passed from host command"
        );
        assert!(
            app.world.get_resource::<RenetClient>().is_some(),
            "Client resource should exist"
        );
    }

    #[test]
    fn connects() {
        let mut app = App::new();
        app.add_plugin(TestClientPlugin::new(NetworkingState::NoSocket))
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: false,
            }));

        app.update();
        app.update();

        let networking_state = app.world.resource::<CurrentState<NetworkingState>>();
        assert!(
            matches!(networking_state.0, NetworkingState::Connecting),
            "Networking state should be in {:?} state after client creation",
            NetworkingState::Connecting,
        );

        app.update();
        app.update();

        assert!(
            app.world.resource::<RenetClient>().is_connected(),
            "Client should be connected",
        );

        let networking_state = app.world.resource::<CurrentState<NetworkingState>>();
        assert!(
            matches!(networking_state.0, NetworkingState::Connected),
            "Networking state should be in {:?} state after connection",
            NetworkingState::Connected,
        );
        app.world
            .insert_resource(NextState(NetworkingState::NoSocket));

        app.update();

        assert!(
            app.world.get_resource::<RenetClient>().is_none(),
            "Client resource should be removed on entering {:?} state",
            NetworkingState::NoSocket,
        );
    }

    #[test]
    fn connection_cancels() {
        let mut app = App::new();
        app.add_plugin(TestClientPlugin::new(NetworkingState::Connecting))
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Client));

        app.world
            .insert_resource(NextState(NetworkingState::NoSocket));

        app.update();

        assert!(
            app.world.get_resource::<RenetClient>().is_none(),
            "Client resource should be removed on entering {:?} state",
            NetworkingState::NoSocket,
        );
    }

    struct TestClientPlugin {
        networking_state: NetworkingState,
    }

    impl TestClientPlugin {
        fn new(networking_state: NetworkingState) -> Self {
            Self { networking_state }
        }
    }

    impl Plugin for TestClientPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<Opts>()
                .add_loopless_state(self.networking_state)
                .add_plugin(ClientPlugin);
        }
    }
}

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

use bevy::prelude::*;
use bevy_renet::renet::{ConnectToken, RenetClient, RenetConnectionConfig};
use clap::Args;
use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use super::{SocketEvent, DEFAULT_PORT, PROTOCOL_ID, PUBLIC_GAME_KEY};
use crate::core::cli::{Opts, SubCommand};

pub(super) struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::socket_events_system);

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
    fn socket_events_system(
        mut client_was_connected: Local<bool>,
        mut socket_events: EventWriter<SocketEvent>,
        client: Option<Res<RenetClient>>,
    ) {
        if let Some(client) = client {
            if client.is_connected() != *client_was_connected {
                socket_events.send(SocketEvent::Opened);
                *client_was_connected = true;
            }
        } else if *client_was_connected {
            socket_events.send(SocketEvent::Closed);
            *client_was_connected = false;
        }
    }
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
            RenetConnectionConfig::default(),
        )
        .map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use bevy_renet::{RenetClientPlugin, RenetServerPlugin};

    use super::*;
    use crate::core::network::server::ServerSettings;

    #[test]
    fn defaulted_without_connect() {
        let mut app = App::new();
        app.init_resource::<Opts>();
        app.add_plugin(ClientPlugin);

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
            port: ConnectionSettings::default().port + 1,
            ..Default::default()
        };
        app.world.insert_resource(Opts {
            subcommand: Some(SubCommand::Connect(connection_settings.clone())),
        });
        app.add_plugin(ClientPlugin);

        assert_eq!(
            *app.world.resource::<ConnectionSettings>(),
            connection_settings,
            "Connection settings should be initialized with parameters passed from host command"
        );
        assert!(
            app.world.get_resource::<RenetClient>().is_some(),
            "Connection should be opened"
        );
    }

    #[test]
    fn socket_events() {
        let mut app = App::new();
        let server_settings = ServerSettings {
            port: ServerSettings::default().port + 2,
            ..Default::default()
        };
        let connection_settings = ConnectionSettings {
            port: server_settings.port,
            ..Default::default()
        };
        app.init_resource::<Opts>()
            .add_event::<SocketEvent>()
            .add_plugins(MinimalPlugins)
            .add_plugin(RenetServerPlugin)
            .add_plugin(RenetClientPlugin)
            .add_plugin(ClientPlugin)
            .insert_resource(
                server_settings
                    .create_server()
                    .expect("Server should be created succesfully from settings"),
            )
            .insert_resource(
                connection_settings
                    .create_client()
                    .expect("Client should be created succesfully from settings"),
            );

        app.update();
        app.update();
        app.update();

        let mut socket_events = app.world.resource_mut::<Events<SocketEvent>>();
        let event = socket_events
            .drain()
            .next()
            .expect("Socket event should be triggered on client connection");

        assert!(
            matches!(event, SocketEvent::Opened),
            "Socket should be opened on client creation"
        );

        app.world.remove_resource::<RenetClient>();

        app.update();

        let mut socket_events = app.world.resource_mut::<Events<SocketEvent>>();
        let event = socket_events
            .drain()
            .next()
            .expect("Socket event should be triggered on client removal");

        assert!(
            matches!(event, SocketEvent::Closed),
            "Socket should be closed on client removal"
        );
    }
}

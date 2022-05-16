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

use super::{DEFAULT_PORT, PROTOCOL_ID, PUBLIC_GAME_KEY};
use crate::core::cli::{Opts, SubCommand};

pub(super) struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
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
    use super::*;

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
}

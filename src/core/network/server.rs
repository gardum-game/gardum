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
use bevy_renet::renet::{RenetConnectionConfig, RenetServer, ServerConfig};
use clap::Args;
use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use super::{Channel, DEFAULT_PORT, PROTOCOL_ID, PUBLIC_GAME_KEY};
use crate::core::{
    cli::{Opts, SubCommand},
    map::Map,
    session::GameMode,
};

pub(super) struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let opts = app
            .world
            .get_resource::<Opts>()
            .expect("Command line options should be initialized before server settings resource");
        if let Some(SubCommand::Host(server_settings)) = &opts.subcommand {
            let settings = server_settings.clone();
            app.insert_resource(settings.create_server().expect("Unable to create server"));
            app.insert_resource(settings);
        } else {
            app.insert_resource(ServerSettings::default());
        }
    }
}

pub(crate) fn random_heroes(server_settings: Res<ServerSettings>) -> bool {
    server_settings.random_heroes
}

#[derive(Args, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct ServerSettings {
    /// Server name that will be visible to other players.
    #[clap(short, long, default_value_t = ServerSettings::default().server_name)]
    pub(crate) server_name: String,

    /// IP address to bind.
    #[clap(short, long, default_value_t = ServerSettings::default().ip)]
    pub(crate) ip: String,

    /// Port to use.
    #[clap(short, long, default_value_t = ServerSettings::default().port)]
    pub(crate) port: u16,

    /// Game mode.
    #[clap(short, long, default_value_t = ServerSettings::default().game_mode)]
    pub(crate) game_mode: GameMode,

    /// Game map.
    #[clap(short, long, default_value_t = ServerSettings::default().map)]
    pub(crate) map: Map,

    /// Choose heroes randomly.
    #[clap(short, long)]
    pub(crate) random_heroes: bool,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            server_name: "My game".to_string(),
            ip: "127.0.0.1".to_string(),
            port: DEFAULT_PORT,
            game_mode: GameMode::Deathmatch,
            map: Map::SkyRoof,
            random_heroes: false,
        }
    }
}

impl ServerSettings {
    pub(crate) fn create_server(&self) -> Result<RenetServer, Box<dyn Error>> {
        let server_addr = SocketAddr::new(self.ip.parse()?, self.port);
        let socket = UdpSocket::bind(server_addr)?;
        RenetServer::new(
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?,
            ServerConfig::new(64, PROTOCOL_ID, socket.local_addr()?, PUBLIC_GAME_KEY),
            RenetConnectionConfig {
                channels_config: Channel::config(),
                ..Default::default()
            },
            socket,
        )
        .map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaulted_without_host() {
        let mut app = App::new();
        app.init_resource::<Opts>();
        app.add_plugin(ServerPlugin);

        assert_eq!(
            *app.world.resource::<ServerSettings>(),
            ServerSettings::default(),
            "Server settings should be initialized with defaults without host command"
        );
        assert!(
            app.world.get_resource::<RenetServer>().is_none(),
            "Server should't be created"
        )
    }

    #[test]
    fn initializes_from_host() {
        let mut app = App::new();
        let server_settings = ServerSettings {
            port: 0,
            ..Default::default()
        };
        app.world.insert_resource(Opts {
            subcommand: Some(SubCommand::Host(server_settings.clone())),
        });
        app.add_plugin(ServerPlugin);

        assert_eq!(
            *app.world.resource::<ServerSettings>(),
            server_settings,
            "Server settings should be initialized with parameters passed from host command"
        );
        assert!(
            app.world.get_resource::<RenetServer>().is_some(),
            "Server resource should exist"
        );
    }
}

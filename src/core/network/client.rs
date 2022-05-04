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
use clap::Args;

use super::DEFAULT_PORT;
use crate::core::cli::{Opts, SubCommand};

pub(super) struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let opts = app
            .world
            .get_resource::<Opts>()
            .expect("Command line options should be initialized before client plugin");

        let connectioin_settings = match &opts.subcommand {
            Some(SubCommand::Connect(connectioin_settings)) => connectioin_settings.clone(),
            _ => ConnectionSettings::default(),
        };
        app.insert_resource(connectioin_settings);
    }
}

#[derive(Args, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct ConnectionSettings {
    /// Server name that will be visible to other players.
    #[clap(short, long, default_value_t = ConnectionSettings::default().ip)]
    pub(crate) ip: String,

    /// Port to use.
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
            "Server settings should be initialized with defaults without host command"
        );
    }

    #[test]
    fn initializes_from_connect() {
        let mut app = App::new();
        let connection_settings = ConnectionSettings {
            ip: "0.0.0.0".to_string(),
            ..Default::default()
        };
        app.world.insert_resource(Opts {
            subcommand: Some(SubCommand::Connect(connection_settings.clone())),
        });
        app.add_plugin(ClientPlugin);

        assert_eq!(
            *app.world.resource::<ConnectionSettings>(),
            connection_settings,
            "Server settings should be initialized with parameters passed from host command"
        );
    }
}

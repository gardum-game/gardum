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

use super::{
    cli::{Opts, SubCommand},
    map::Map,
    session::GameMode,
};

pub(super) struct ServerSettingsPlugin;

impl Plugin for ServerSettingsPlugin {
    fn build(&self, app: &mut App) {
        let opts = app
            .world
            .get_resource::<Opts>()
            .expect("Command line options should be initialized before server settings resource");

        let server_settings = match &opts.subcommand {
            Some(SubCommand::Host(server_settings)) => server_settings.clone(),
            _ => ServerSettings::default(),
        };
        app.insert_resource(server_settings);
    }
}

#[derive(Args, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct ServerSettings {
    /// Server name that will be visible to other players.
    #[clap(short, long, default_value_t = ServerSettings::default().server_name)]
    pub(crate) server_name: String,

    /// Port to use.
    #[clap(short, long, default_value_t = ServerSettings::default().port)]
    pub(crate) port: u16,

    /// Game mode.
    #[clap(short, long)]
    pub(crate) game_mode: GameMode,

    /// Game map.
    #[clap(short, long)]
    pub(crate) map: Map,

    /// Choose heroes randomly.
    #[clap(short, long)]
    pub(crate) random_heroes: bool,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            server_name: "My game".to_string(),
            port: 4761,
            game_mode: GameMode::Deathmatch,
            map: Map::SkyRoof,
            random_heroes: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaulted_without_host() {
        let mut app = App::new();
        app.init_resource::<Opts>();
        app.add_plugin(ServerSettingsPlugin);

        assert_eq!(
            *app.world.resource::<ServerSettings>(),
            ServerSettings::default(),
            "Server settings should be initialized with defaults without host command"
        );
    }

    #[test]
    fn initializes_from_host() {
        let mut app = App::new();
        let server_settings = ServerSettings {
            random_heroes: !ServerSettings::default().random_heroes,
            ..Default::default()
        };
        app.world.insert_resource(Opts {
            subcommand: Some(SubCommand::Host(server_settings.clone())),
        });
        app.add_plugin(ServerSettingsPlugin);

        assert_eq!(
            *app.world.resource::<ServerSettings>(),
            server_settings,
            "Server settings should be initialized with parameters passed from host command"
        );
    }
}

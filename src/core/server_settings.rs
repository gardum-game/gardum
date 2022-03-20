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

use super::cli::{Opts, SubCommand};

#[derive(Args, Clone)]
pub(crate) struct ServerSettings {
    /// Server name that will be visible to other players.
    #[clap(short, long, default_value_t = ServerSettings::default().game_name)]
    pub(crate) game_name: String,

    /// Port to use.
    #[clap(short, long, default_value_t = ServerSettings::default().port)]
    pub(crate) port: u16,

    /// Port to use.
    #[clap(short, long)]
    pub(crate) random_heroes: bool,
}

impl ServerSettings {
    /// We do not use the [`Default`] trait to avoid conflicting [`FromWorld`] implementation.
    pub(super) fn default() -> Self {
        Self {
            game_name: "My game".to_string(),
            port: 4761,
            random_heroes: false,
        }
    }
}

impl FromWorld for ServerSettings {
    fn from_world(world: &mut World) -> Self {
        let opts = world
            .get_resource::<Opts>()
            .expect("Command line options should be initialized before server settings resource");

        if let Some(SubCommand::Host(server_settings)) = &opts.subcommand {
            server_settings.clone()
        } else {
            ServerSettings::default()
        }
    }
}

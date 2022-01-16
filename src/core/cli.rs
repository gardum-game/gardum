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
use clap::{Parser, Subcommand};

pub(super) struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(test) {
            // Dont parse command line when tarpaulin is used
            app.init_resource::<Opts>();
        } else {
            app.insert_resource(Opts::parse());
        }
    }
}

#[derive(Default, Parser)]
#[clap(author, version, about)]
pub(super) struct Opts {
    #[clap(subcommand)]
    pub(super) subcommand: Option<SubCommand>,
}

#[derive(Subcommand)]
pub(super) enum SubCommand {
    Connect,
    Host,
}

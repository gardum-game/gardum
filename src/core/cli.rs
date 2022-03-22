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

use clap::{Parser, Subcommand};

use super::server_settings::ServerSettings;

#[derive(Parser)]
#[clap(author, version, about)]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub(crate) subcommand: Option<SubCommand>,
}

impl Default for Opts {
    fn default() -> Self {
        if cfg!(test) {
            // Do not parse command line in tests
            Opts { subcommand: None }
        } else {
            Opts::parse()
        }
    }
}

#[derive(Subcommand)]
pub(crate) enum SubCommand {
    Connect,
    Host(ServerSettings),
}

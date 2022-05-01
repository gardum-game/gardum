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

pub(super) mod spawn;

use bevy::prelude::*;
use strum::{EnumIter, EnumString};

use spawn::SpawnPlugin;

pub(super) struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpawnPlugin);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter, EnumString)]
pub(crate) enum GameMode {
    Deathmatch,
}

impl GameMode {
    pub(crate) const fn slots_count(self) -> u8 {
        match self {
            GameMode::Deathmatch => 10,
        }
    }
}

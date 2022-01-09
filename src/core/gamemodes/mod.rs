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
use strum::EnumIter;

pub struct GamemodesPlugin;

impl Plugin for GamemodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameMode::Disabled);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub enum GameMode {
    Disabled,
    Deathmatch,
}

impl GameMode {
    pub const fn slots_count(self) -> u8 {
        match self {
            GameMode::Disabled | GameMode::Deathmatch => 10,
        }
    }
}

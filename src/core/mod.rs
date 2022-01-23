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

mod cli;
pub(super) mod player;

use bevy::prelude::*;
use derive_more::From;
use heron::PhysicsLayer;
#[cfg(test)]
use strum::EnumIter;

use cli::CliPlugin;
use player::PlayerPlugin;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu)
            .init_resource::<ServerSettings>()
            .add_plugin(CliPlugin)
            .add_plugin(PlayerPlugin);
    }
}

#[derive(Default, Component)]
pub(super) struct Authority;

pub(super) struct ServerSettings {
    pub(super) game_name: String,
    pub(super) port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            game_name: "My game".to_string(),
            port: 4761,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(EnumIter))]
pub(super) enum AppState {
    Menu,
    Lobby,
    InGame,
}

#[derive(PhysicsLayer)]
pub(super) enum CollisionLayer {
    Character,
    Projectile,
}

/// Path to icon resource.
#[derive(Component, From)]
pub(super) struct IconPath(pub(super) &'static str);

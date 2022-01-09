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

pub mod cli;
pub mod gamemodes;
pub mod player;
mod setup;

use bevy::prelude::*;
use derive_more::From;
use heron::PhysicsLayer;

use cli::CliPlugin;
use gamemodes::{GameMode, GamemodesPlugin};
use player::PlayerPlugin;
use setup::SetupPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::MainMenu)
            .init_resource::<GameSettings>()
            .add_plugin(CliPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GamemodesPlugin)
            .add_plugin(SetupPlugin);
    }
}

#[derive(Default, Component)]
pub struct Authority;

pub struct GameSettings {
    pub game_name: String,
    pub port: u16,
    pub game_mode: GameMode,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            game_name: "My game".to_string(),
            port: 4761,
            game_mode: GameMode::Deathmatch,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    CustomGameMenu,
    DirectConnectMenu,
    CreateGameMenu,
    LobbyMenu,
    InGame,
    InGameMenu,
}

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Character,
    Projectile,
}

/// Path to icon resource.
#[derive(Component, From)]
pub struct IconPath(pub &'static str);

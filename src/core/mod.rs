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

pub(super) mod ability;
pub(super) mod character;
pub(super) mod character_action;
mod cli;
pub(super) mod cooldown;
mod despawn_timer;
mod effect_timer;
pub(super) mod health;
pub(super) mod map;
mod movement;
mod orbit_camera;
pub(super) mod player;
mod projectile;
pub(super) mod session;

use bevy::prelude::*;
use heron::PhysicsLayer;
#[cfg(test)]
use strum::EnumIter;

use ability::AbilityPlugin;
use character::CharactersPlugin;
use character_action::CharacterActionPlugin;
use cli::CliPlugin;
use cooldown::CooldownPlugin;
use despawn_timer::DespawnTimerPlugin;
use health::HealthPlugin;
use map::MapsPlugin;
use movement::MovementPlugin;
use orbit_camera::OrbitCameraPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use session::SessionPlugin;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu)
            .init_resource::<ServerSettings>()
            .add_plugin(HealthPlugin)
            .add_plugin(CharactersPlugin)
            .add_plugin(CharacterActionPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(CooldownPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(CliPlugin)
            .add_plugin(MapsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SessionPlugin)
            .add_plugin(DespawnTimerPlugin)
            .add_plugin(ProjectilePlugin);
    }
}

/// Indicates that the local player have authority on the entity
#[derive(Default, Component)]
pub(super) struct Local;

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

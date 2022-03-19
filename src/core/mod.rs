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
pub(super) mod cli;
pub(super) mod cooldown;
mod despawn_timer;
mod effect;
pub(super) mod health;
pub(super) mod map;
mod movement;
mod orbit_camera;
mod pickup;
pub(super) mod player;
mod projectile;
pub(super) mod session;

use bevy::{ecs::system::SystemParam, prelude::*};
use clap::Args;
use derive_more::From;
use heron::PhysicsLayer;
#[cfg(test)]
use strum::EnumIter;

use ability::AbilityPlugin;
use character::CharactersPlugin;
use character_action::CharacterActionPlugin;
use cli::CliPlugin;
use despawn_timer::DespawnTimerPlugin;
use effect::EffectPlugin;
use health::HealthPlugin;
use map::MapsPlugin;
use movement::MovementPlugin;
use orbit_camera::OrbitCameraPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use session::SessionPlugin;

use self::cli::{Opts, SubCommand};

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu)
            .add_plugin(CliPlugin)
            .init_resource::<ServerSettings>()
            .add_plugin(HealthPlugin)
            .add_plugin(CharactersPlugin)
            .add_plugin(CharacterActionPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(PickupPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(MapsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SessionPlugin)
            .add_plugin(DespawnTimerPlugin)
            .add_plugin(EffectPlugin)
            .add_plugin(ProjectilePlugin);
    }
}

/// Indicates that the local player have authority on the entity
#[derive(Component)]
pub(super) struct Authority;

#[derive(Args, Clone)]
pub(super) struct ServerSettings {
    /// Server name that will be visible to other players.
    #[clap(short, long, default_value_t = ServerSettings::default().game_name)]
    pub(super) game_name: String,

    /// Port to use.
    #[clap(short, long, default_value_t = ServerSettings::default().port)]
    pub(super) port: u16,

    /// Port to use.
    #[clap(short, long)]
    pub(super) random_heroes: bool,
}

impl ServerSettings {
    /// We do not use the [`Default`] trait to avoid conflicting [`FromWorld`] implementation.
    fn default() -> Self {
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
    Pickup,
}

/// Used to store reference to the owner
#[derive(Component, From)]
struct Owner(Entity);

/// TODO 0.7: Replace with built-in
#[derive(Bundle, Clone, Copy, Debug, Default)]
struct TransformBundle {
    pub local: Transform,
    pub global: GlobalTransform,
}

impl TransformBundle {
    /// Creates a new [`TransformBundle`] from a [`Transform`].
    ///
    /// This initializes [`GlobalTransform`] as identity, to be updated later by the
    /// [`CoreStage::PostUpdate`](crate::CoreStage::PostUpdate) stage.
    #[inline]
    pub fn from_transform(transform: Transform) -> Self {
        TransformBundle {
            local: transform,
            ..Self::default()
        }
    }
}

/// Helper for easier asset spawning
#[derive(SystemParam)]
struct AssetCommands<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
}

/// Trait to map enumerations with associated assets
trait AssociatedAsset {
    /// Returns path to associated asset
    fn asset_path(&self) -> &str;
}

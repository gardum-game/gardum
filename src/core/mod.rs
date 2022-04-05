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
pub(super) mod cli;
pub(super) mod cooldown;
mod despawn_timer;
mod effect;
pub(super) mod game_state;
pub(super) mod health;
pub(super) mod map;
mod movement;
mod orbit_camera;
mod pickup;
pub(super) mod player;
pub(super) mod server_settings;
pub(super) mod session;
pub(super) mod settings;

use bevy::{ecs::system::SystemParam, prelude::*};
use derive_more::From;
use heron::{CollisionLayers, CollisionShape, Collisions, PhysicsLayer, RigidBody, Velocity};

use ability::AbilityPlugin;
use character::CharactersPlugin;
use cli::Opts;
use despawn_timer::DespawnTimer;
use despawn_timer::DespawnTimerPlugin;
use effect::EffectPlugin;
use game_state::AppStatePlugin;
use health::HealthPlugin;
use map::MapsPlugin;
use movement::MovementPlugin;
use orbit_camera::OrbitCameraPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use server_settings::ServerSettingsPlugin;
use session::SessionPlugin;
use settings::SettingsPlugin;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Opts>()
            .add_plugin(ServerSettingsPlugin)
            .add_plugin(SettingsPlugin)
            .add_plugin(AppStatePlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(CharactersPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(PickupPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(MapsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SessionPlugin)
            .add_plugin(DespawnTimerPlugin)
            .add_plugin(EffectPlugin);
    }
}

/// Indicates that the local player have authority on the entity
#[derive(Component)]
pub(super) struct Authority;

#[derive(PhysicsLayer)]
pub(super) enum CollisionLayer {
    World,
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

#[derive(Bundle)]
pub(super) struct ProjectileBundle {
    name: Name,
    rigid_body: RigidBody,
    shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,
    despawn_timer: DespawnTimer,
    collisions: Collisions,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            name: "Projectile".into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::all_masks::<CollisionLayer>()
                .without_mask(CollisionLayer::Projectile)
                .with_group(CollisionLayer::Projectile),
            velocity: Velocity::default(),
            despawn_timer: DespawnTimer::from_secs(4),
            collisions: Collisions::default(),
            pbr: PbrBundle::default(),
        }
    }
}

/// Helper for easier asset spawning
#[derive(SystemParam)]
struct AssetCommands<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
}

/// Trait to map enumerations with associated assets
trait AssociatedAsset {
    /// Returns path to associated asset
    fn asset_path(&self) -> &str;
}

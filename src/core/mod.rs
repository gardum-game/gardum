/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

pub(super) mod ability;
pub(super) mod character;
pub(super) mod cli;
pub(super) mod control_actions;
pub(super) mod cooldown;
mod despawn_timer;
#[cfg(feature = "developer")]
mod developer;
mod effect;
pub(super) mod game_state;
mod graphics;
#[cfg(test)]
pub(super) mod headless;
pub(super) mod health;
pub(super) mod map;
mod movement;
pub(super) mod network;
mod orbit_camera;
mod pickup;
pub(super) mod player;
pub(super) mod session;
pub(super) mod settings;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::*;
use bitflags::bitflags;
use derive_more::From;

use ability::AbilityPlugin;
use character::CharactersPlugin;
use cli::Opts;
use control_actions::ControlActionsPlugin;
use despawn_timer::DespawnTimer;
use despawn_timer::DespawnTimerPlugin;
#[cfg(feature = "developer")]
use developer::DeveloperPlugin;
use effect::EffectPlugin;
use game_state::AppStatePlugin;
use game_state::InGameOnly;
use health::HealthPlugin;
use map::MapsPlugin;
use movement::MovementPlugin;
use network::NetworkPlugin;
use orbit_camera::OrbitCameraPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use session::SessionPlugin;
use settings::SettingsPlugin;

pub(super) struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Opts>()
            .add_plugin(NetworkPlugin)
            .add_plugin(SettingsPlugin)
            .add_plugin(AppStatePlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(CharactersPlugin)
            .add_plugin(ControlActionsPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(PickupPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(MapsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SessionPlugin)
            .add_plugin(DespawnTimerPlugin)
            .add_plugin(EffectPlugin);

        #[cfg(feature = "developer")]
        app.add_plugin(DeveloperPlugin);
    }
}

/// Indicates that the local player have authority on the entity
#[derive(Component)]
pub(super) struct Authority;

bitflags! {
    struct CollisionMask: u32 {
        const WORLD = 0b00000001;
        const CHARACTER = 0b00000010;
        const PROJECTILE = 0b00000100;
        const PICKUP = 0b00001000;
    }
}

/// Used to store reference to the owner
#[derive(Component, From)]
struct Owner(Entity);

#[derive(Bundle)]
pub(super) struct ProjectileBundle {
    name: Name,
    rigid_body: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
    velocity: Velocity,
    despawn_timer: DespawnTimer,
    colliding_entities: CollidingEntities,
    active_events: ActiveEvents,
    ingame_only: InGameOnly,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            name: "Projectile".into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: Collider::capsule_y(0.5, 0.5),
            collision_groups: CollisionGroups {
                memberships: CollisionMask::PROJECTILE.bits(),
                filters: (CollisionMask::all() ^ CollisionMask::PROJECTILE).bits(),
            },
            velocity: Velocity::default(),
            despawn_timer: DespawnTimer::from_secs(4),
            colliding_entities: CollidingEntities::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            ingame_only: InGameOnly,
            pbr: PbrBundle::default(),
        }
    }
}

/// Helper for easier asset spawning
#[derive(SystemParam)]
struct AssetCommands<'w, 's> {
    commands: Commands<'w, 's>,
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
}

/// Trait to map enumerations with associated assets
trait AssociatedAsset {
    /// Returns path to associated asset
    fn asset_path(&self) -> &str;
}

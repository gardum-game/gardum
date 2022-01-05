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

pub mod ability;
pub mod camera;
pub mod cooldown;
pub mod despawn_timer;
pub mod health;
pub mod heroes;
pub mod movement;
pub mod projectile;

use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, Velocity};

use crate::core::CollisionLayer;
use ability::AbilityPlugin;
use camera::CameraPlugin;
use cooldown::CooldownPlugin;
use despawn_timer::DespawnTimerPlugin;
use health::HealthPlugin;
use heroes::HeroesPlugin;
use movement::MovementPlugin;
use projectile::ProjectilePlugin;

use self::health::Health;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MovementPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(CooldownPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(DespawnTimerPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(HeroesPlugin)
            .add_plugin(ProjectilePlugin);
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    health: Health,
    rigid_body: RigidBody,
    shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            health: Health::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::all::<CollisionLayer>()
                .with_group(CollisionLayer::Character),
            velocity: Velocity::default(),
            pbr: PbrBundle::default(),
        }
    }
}

/// Used to store reference to the character
pub struct CharacterOwner(pub Entity);

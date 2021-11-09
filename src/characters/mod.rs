/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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

mod abilities;
mod camera;
pub mod heroes;
mod movement;
pub mod projectile;

use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, Velocity};

use crate::core::CollisionLayer;
use abilities::AbilitiesPlugin;
use camera::CameraPlugin;
use heroes::HeroesPlugin;
use movement::MovementPlugin;
use projectile::ProjectilePlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MovementPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(AbilitiesPlugin)
            .add_plugin(HeroesPlugin)
            .add_plugin(ProjectilePlugin);
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
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
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::all::<CollisionLayer>()
                .with_group(CollisionLayer::Character),
            velocity: Velocity::default(),
            pbr: PbrBundle::default(),
        }
    }
}

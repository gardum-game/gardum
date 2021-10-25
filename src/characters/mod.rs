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

use bevy::prelude::*;
use heron::{CollisionShape, RigidBody, Velocity};

use abilities::Abilities;
use abilities::AbilitiesPlugin;
use camera::CameraPlugin;
pub use heroes::HeroAssets;
use heroes::HeroesPlugin;
use movement::MovementPlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MovementPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(AbilitiesPlugin)
            .add_plugin(HeroesPlugin);
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    rigid_body: RigidBody,
    shape: CollisionShape,
    velocity: Velocity,
    abilities: Abilities,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            velocity: Velocity::default(),
            abilities: Abilities::default(),
            pbr: PbrBundle::default(),
        }
    }
}

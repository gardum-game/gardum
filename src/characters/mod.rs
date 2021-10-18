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

mod camera;
pub mod heroes;
mod movement;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    ColliderBundle, RigidBodyBundle, RigidBodyPositionSync, RigidBodyType,
};

use camera::CameraPlugin;
pub use heroes::HeroAssets;
use heroes::HeroesPlugin;
use movement::MovementPlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(MovementPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(HeroesPlugin);
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    position_sync: RigidBodyPositionSync,

    #[bundle]
    pbr: PbrBundle,

    #[bundle]
    collider: ColliderBundle,

    #[bundle]
    rigid_body: RigidBodyBundle,
}

impl CharacterBundle {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>, position: Vec3) -> Self {
        Self {
            position_sync: RigidBodyPositionSync::Discrete,
            pbr: PbrBundle {
                mesh,
                material,
                ..Default::default()
            },
            collider: Default::default(),
            rigid_body: RigidBodyBundle {
                body_type: RigidBodyType::KinematicVelocityBased,
                position: position.into(),
                ..Default::default()
            },
        }
    }
}

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

use super::HeroAssets;
use crate::characters::CharacterBundle;
use bevy::prelude::*;

pub struct DummyAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl DummyAssets {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        }
    }
}

impl CharacterBundle {
    pub fn dummy(assets: &HeroAssets, position: Vec3) -> Self {
        Self::new(
            assets.dummy.mesh.clone(),
            assets.dummy.material.clone(),
            position,
        )
    }
}

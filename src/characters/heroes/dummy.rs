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

use bevy::prelude::*;
use heron::CollisionShape;

use crate::characters::abilities::Ability;
use crate::characters::CharacterBundle;

impl CharacterBundle {
    pub fn dummy(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        transform: Transform,
    ) -> Self {
        Self {
            abilities: Vec::from([Ability::frost_bolt()]).into(),
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform,
                ..Default::default()
            },
            shape: CollisionShape::Capsule {
                half_segment: 0.5,
                radius: 0.5,
            },
            ..Default::default()
        }
    }
}

impl Ability {
    fn frost_bolt() -> Self {
        Self::new(frost_bolt_ability, 0)
    }
}

fn frost_bolt_ability() {
    println!("Called!");
}

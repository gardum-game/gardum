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

mod dummy;

use bevy::prelude::*;

use dummy::DummyAssets;
pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<HeroAssets>();
    }
}

pub struct HeroAssets {
    dummy: DummyAssets,
}

impl FromWorld for HeroAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            dummy: DummyAssets::new(&mut meshes, &mut materials),
        }
    }
}

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

mod sky_roof;

use bevy::prelude::*;

use strum::EnumIter;

use super::AssetCommands;
use crate::core::AppState;

pub(super) struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::Plane)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(load_map_system));
    }
}

fn load_map_system(map: Res<Map>, mut asset_commands: AssetCommands) {
    match *map {
        Map::Plane => asset_commands.spawn_sky_roof(),
    };
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub(crate) enum Map {
    Plane,
}

#[cfg(test)]
mod tests {
    use bevy::{gltf::GltfPlugin, scene::ScenePlugin};
    use strum::IntoEnumIterator;

    use super::*;
    use crate::test_utils::HeadlessRenderPlugin;

    #[test]
    fn initialization_on_start() {
        let mut app = setup_app();
        app.add_state(AppState::InGame);

        let mut meshes_query = app.world.query::<&Handle<Mesh>>();
        for map in Map::iter() {
            let mut current_map = app.world.get_resource_mut::<Map>().unwrap();
            *current_map = map;

            const MAX_UPDATES: u8 = 25;
            let mut updates_count = 1;
            loop {
                app.update();
                if meshes_query.iter(&app.world).count() > 0 || updates_count == MAX_UPDATES {
                    break;
                }
                updates_count += 1;
            }

            assert_ne!(updates_count, MAX_UPDATES, "Map meshes should be loaded");

            app.world.clear_entities();
        }
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin)
            .add_plugin(ScenePlugin)
            .add_plugin(GltfPlugin)
            .add_plugin(TransformPlugin)
            .add_plugin(MapsPlugin);
        app
    }
}

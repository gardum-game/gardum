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

use strum::{Display, EnumIter, EnumString};

use super::{server_settings::ServerSettings, AssetCommands, AssociatedAsset};
use crate::core::game_state::GameState;

pub(super) struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(load_map_system));
    }
}

fn load_map_system(server_settings: Res<ServerSettings>, mut asset_commands: AssetCommands) {
    match server_settings.map {
        Map::SkyRoof => asset_commands.spawn_sky_roof(),
    };
}

#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, PartialEq)]
pub(crate) enum Map {
    SkyRoof,
}

impl AssociatedAsset for Map {
    fn asset_path(&self) -> &str {
        match self {
            Map::SkyRoof => "maps/sky_roof.glb#Scene0",
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{gltf::GltfPlugin, scene::ScenePlugin};

    use super::*;
    use crate::test_utils::{wait_for_asset_loading, HeadlessRenderPlugin};

    #[test]
    fn loading_on_start() {
        let mut app = setup_app();
        let map = app.world.resource::<ServerSettings>().map;

        wait_for_asset_loading(&mut app, map.asset_path(), 25);

        app.world.clear_entities();
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(GameState::InGame)
            .init_resource::<ServerSettings>()
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(HierarchyPlugin)
            .add_plugin(ScenePlugin)
            .add_plugin(GltfPlugin)
            .add_plugin(TransformPlugin)
            .add_plugin(MapsPlugin);
        app
    }
}

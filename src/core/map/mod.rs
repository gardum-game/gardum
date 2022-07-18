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

mod sky_roof;

use bevy::prelude::*;
use strum::{Display, EnumIter, EnumString};

use super::AssociatedAsset;
use sky_roof::SkyRoofPlugin;

pub(super) struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SkyRoofPlugin);
    }
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
    use iyes_loopless::prelude::*;

    use super::*;
    use crate::core::{
        game_state::GameState,
        headless::{self, HeadlessRenderPlugin},
        network::server::ServerSettings,
    };

    #[test]
    fn loading_on_start() {
        let mut app = App::new();
        app.add_plugin(TestMapPlugin);

        let map = app.world.resource::<ServerSettings>().map;

        headless::wait_for_asset_loading(&mut app, map.asset_path());

        app.world.clear_entities();
    }

    struct TestMapPlugin;

    impl Plugin for TestMapPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(GameState::InGame)
                .init_resource::<ServerSettings>()
                .add_plugin(HeadlessRenderPlugin)
                .add_plugin(HierarchyPlugin)
                .add_plugin(ScenePlugin)
                .add_plugin(GltfPlugin)
                .add_plugin(TransformPlugin)
                .add_plugin(MapsPlugin);
        }
    }
}

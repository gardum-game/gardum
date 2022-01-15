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

mod plane;

use bevy::prelude::*;
use strum::EnumIter;

use crate::core::AppState;

pub(super) struct MapsPlugin;

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::Plane)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(load_map_system));
    }
}

fn load_map_system(
    map: Res<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    map.setup(&mut commands, &mut meshes, &mut materials);
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub(super) enum Map {
    Plane,
}

impl Map {
    fn setup(
        self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        let setup_fn = match self {
            Map::Plane => Self::plane,
        };

        setup_fn(commands, meshes, materials);
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::SystemState;
    use strum::IntoEnumIterator;

    use super::*;
    use crate::test_utils::HeadlessRenderPlugin;

    #[test]
    fn initialization_on_start() {
        let mut app = setup_app();
        app.add_state(AppState::InGame);

        assert_eq!(
            app.world.entities().len(),
            0,
            "Should be zero entities before update"
        );
        app.update();
        assert!(
            app.world.entities().len() > 0,
            "Map should be initialized after first update"
        );
    }

    #[test]
    fn setup() {
        let mut app = setup_app();
        let mut system_state: SystemState<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
        )> = SystemState::new(&mut app.world);
        let (mut commands, mut meshes, mut materials) = system_state.get_mut(&mut app.world);

        for map in Map::iter() {
            map.setup(&mut commands, &mut meshes, &mut materials);
        }
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin).add_plugin(MapsPlugin);
        app
    }
}

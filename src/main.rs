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

#![allow(clippy::type_complexity)] // Do not warn about long queries

mod core;
#[cfg(test)]
mod test_utils;
#[cfg(feature = "client")]
mod ui;

use bevy::{prelude::*, winit::WinitPlugin};
#[cfg(feature = "client")]
use bevy_egui::EguiPlugin;
use heron::PhysicsPlugin;

use crate::core::CorePlugin;
#[cfg(feature = "client")]
use {
    crate::core::character_action::CharacterAction,
    bevy_atmosphere::AtmospherePlugin,
    bevy_hikari::VoxelConeTracingPlugin,
    leafwing_input_manager::prelude::InputManagerPlugin,
    ui::ui_state::UiState,
    ui::{ui_action::UiAction, UiPlugin},
};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut app = App::new();
    if cfg!(feature = "client") {
        app.add_plugins(DefaultPlugins);
    } else {
        app.add_plugins_with(DefaultPlugins, |group| group.disable::<WinitPlugin>());
    }

    app.add_plugin(PhysicsPlugin::default())
        .add_plugin(CorePlugin);

    #[cfg(feature = "client")]
    app.add_plugin(EguiPlugin)
        .add_plugin(VoxelConeTracingPlugin)
        .add_plugin(AtmospherePlugin::default())
        .add_plugin(InputManagerPlugin::<CharacterAction, UiState>::run_in_state(UiState::Hud))
        .add_plugin(InputManagerPlugin::<UiAction>::default())
        .add_plugin(UiPlugin);

    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}

#[cfg(test)]
mod tests {
    use bevy::{input::InputPlugin, scene::ScenePlugin};
    use strum::IntoEnumIterator;
    use test_utils::HeadlessRenderPlugin;

    use super::*;
    use crate::core::AppState;

    #[test]
    fn update_in_states() {
        let mut app = setup_app();
        app.update();

        for state in AppState::iter().skip(1) {
            let mut current_state = app.world.get_resource_mut::<State<AppState>>().unwrap();
            current_state.set(state).unwrap();
            app.update();
        }
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(ScenePlugin)
            .add_plugin(CorePlugin);

        app
    }
}

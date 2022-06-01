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
use bevy_rapier3d::prelude::*;
use bevy_renet::RenetServerPlugin;

use crate::core::CorePlugin;
#[cfg(feature = "client")]
use {
    crate::core::control_actions::ControlAction,
    bevy::diagnostic::FrameTimeDiagnosticsPlugin,
    bevy_atmosphere::AtmospherePlugin,
    bevy_renet::RenetClientPlugin,
    leafwing_input_manager::prelude::InputManagerPlugin,
    ui::{ui_actions::UiAction, UiPlugin},
};

#[cfg(feature = "developer")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut app = App::new();
    if cfg!(feature = "client") {
        app.add_plugins(DefaultPlugins);
    } else {
        app.add_plugins_with(DefaultPlugins, |group| group.disable::<WinitPlugin>());
    }

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(CorePlugin)
        .add_plugin(RenetServerPlugin);

    #[cfg(feature = "client")]
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(AtmospherePlugin {
            dynamic: false,
            sky_radius: 100.0,
        })
        .add_plugin(InputManagerPlugin::<ControlAction>::default())
        .add_plugin(InputManagerPlugin::<UiAction>::default())
        .add_plugin(RenetClientPlugin)
        .add_plugin(UiPlugin);

    #[cfg(feature = "developer")]
    app.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierDebugRenderPlugin::default());

    app.run();
}

#[cfg(test)]
mod tests {
    use bevy::{input::InputPlugin, scene::ScenePlugin};
    use test_utils::HeadlessRenderPlugin;

    use super::*;

    #[test]
    fn plugins_initialization() {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(ScenePlugin)
            .add_plugin(CorePlugin);
    }
}

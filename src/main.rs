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

mod characters;
mod core;
mod maps;
#[cfg(test)]
mod test_utils;
#[cfg(feature = "client")]
mod ui;

use bevy::{prelude::*, winit::WinitPlugin};
#[cfg(feature = "client")]
use bevy_egui::EguiPlugin;
use heron::PhysicsPlugin;

use crate::core::CorePlugin;
use characters::CharactersPlugin;
use maps::MapsPlugin;
#[cfg(feature = "client")]
use ui::UiPlugin;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut app = App::new();
    if cfg!(feature = "client") {
        app.add_plugins(DefaultPlugins);
    } else {
        app.add_plugins_with(DefaultPlugins, |group| group.disable::<WinitPlugin>());
    }

    app.add_plugin(PhysicsPlugin::default())
        .add_plugin(CorePlugin)
        .add_plugin(MapsPlugin)
        .add_plugin(CharactersPlugin);

    #[cfg(feature = "client")]
    app.add_plugin(EguiPlugin).add_plugin(UiPlugin);

    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::HeadlessRenderPlugin;

    #[test]
    fn update() {
        let mut app = setup_app();
        app.update();
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin)
            .add_plugin(CorePlugin)
            .add_plugin(MapsPlugin)
            .add_plugin(CharactersPlugin);

        app
    }
}

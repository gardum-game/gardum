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
use bevy_egui::EguiPlugin;
use bevy_rapier3d::physics::{NoUserData, RapierPhysicsPlugin};

mod main_menu;
use main_menu::MainMenuPlugin;

mod setup;
use setup::SetupPlugin;

mod player_controller;
use player_controller::PlayerControllerPlugin;

mod app_state;
use app_state::AppStatePlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(AppStatePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerControllerPlugin)
        .add_plugin(EguiPlugin)
        .run();
}

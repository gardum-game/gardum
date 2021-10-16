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

mod characters;
mod core;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::physics::{NoUserData, RapierPhysicsPlugin};

use crate::core::CorePlugin;
use characters::CharactersPlugin;
use ui::UiPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(EguiPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(CharactersPlugin)
        .run();
}

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

use bevy::{
    asset::AssetPlugin,
    core::CorePlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::{options::WgpuOptions, RenderPlugin},
    window::WindowPlugin,
};

// Allows to run tests for systems containing rendering related things without GPU
pub struct HeadlessRenderPlugin;

impl Plugin for HeadlessRenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WgpuOptions {
            backends: None,
            ..Default::default()
        })
        .add_plugin(CorePlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(PbrPlugin::default());
    }
}

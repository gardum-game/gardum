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

use crate::core::network::{
    client::ConnectionSettings, server::ServerSettings, DEFAULT_PORT, MAX_PORT,
};
use bevy::{
    asset::{AssetPlugin, LoadState},
    core::CorePlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    window::WindowPlugin,
};
use bevy_renet::{renet::RenetClient, RenetClientPlugin, RenetServerPlugin};
use parking_lot::Mutex;
use std::ops::Range;

/// To use known different ports for different test
pub(super) static AVAILABLE_PORT: Mutex<Range<u16>> =
    parking_lot::const_mutex(DEFAULT_PORT..MAX_PORT);

// Allows to run tests for systems containing rendering related things without GPU
pub(super) struct HeadlessRenderPlugin;

impl Plugin for HeadlessRenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WgpuSettings {
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

/// Creates server and client and initializes connection between them
pub(super) struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        let server_settings = ServerSettings {
            port: AVAILABLE_PORT
                .lock()
                .next()
                .expect("No available empty ports left"),
            ..Default::default()
        };
        let connection_settings = ConnectionSettings {
            port: server_settings.port,
            ..Default::default()
        };

        app.add_plugins(MinimalPlugins)
            .add_plugin(RenetServerPlugin)
            .add_plugin(RenetClientPlugin)
            .insert_resource(
                server_settings
                    .create_server()
                    .expect("Server should be created succesfully from settings"),
            )
            .insert_resource(
                connection_settings
                    .create_client()
                    .expect("Client should be created succesfully from settings"),
            );

        app.update();
        app.update();
        app.update();

        let client = app.world.resource::<RenetClient>();
        assert!(
            client.is_connected(),
            "The client must be connected to the server to send messages",
        );
    }
}

pub(super) fn wait_for_asset_loading(app: &mut App, path: &str, max_updates: u8) {
    let asset_server = app.world.resource::<AssetServer>();
    let handle: Handle<Scene> = asset_server.load(path);

    for _ in 0..max_updates {
        app.update();
        let asset_server = app.world.resource::<AssetServer>();
        match asset_server.get_load_state(handle.clone()) {
            LoadState::Loaded => return,
            LoadState::Failed => panic!("Unable to load {path}"),
            _ => {}
        }
    }
    panic!("Unable to load asset {path} with {max_updates} app updates");
}

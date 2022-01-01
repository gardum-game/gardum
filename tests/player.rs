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

use gardum::core::{
    cli::{HostSubcommand, Opts, SubCommand},
    player::PlayerPlugin,
    AppState, Authority,
};

#[test]
fn server_player_spawns_in_lobby() {
    let mut app = setup_app_in_lobby();
    app.update();

    let mut query = app.world.query_filtered::<(), With<Authority>>();
    query
        .iter(&mut app.world)
        .next()
        .expect("Player component should be spawned"); // TODO 0.6: Use single and remove mutability
}

#[test]
fn server_player_spawns_with_host_command() {
    let mut app = setup_app_with_host_command();
    app.update();

    let mut query = app.world.query_filtered::<(), With<Authority>>();
    query
        .iter(&mut app.world)
        .next()
        .expect("Player component should be spawned"); // TODO 0.6: Use single and remove mutability
}

fn setup_app_in_lobby() -> App {
    let mut app_builder = App::build();
    app_builder
        .init_resource::<Opts>()
        .add_state(AppState::LobbyMenu)
        .add_plugin(PlayerPlugin);
    app_builder.app
}

fn setup_app_with_host_command() -> App {
    let mut app_builder = App::build();
    app_builder
        .insert_resource(Opts {
            subcommand: Some(SubCommand::Host(HostSubcommand {})),
        })
        .add_state(AppState::LobbyMenu)
        .add_plugin(PlayerPlugin);
    app_builder.app
}

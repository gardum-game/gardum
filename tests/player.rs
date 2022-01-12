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

use bevy::prelude::*;

use gardum::{
    characters::heroes::OwnerPlayer,
    core::{
        cli::{Opts, SubCommand},
        player::{PlayerBundle, PlayerHero, PlayerPlugin},
        AppState, Authority,
    },
};

#[test]
fn player_hero_updates() {
    let mut app = setup_app();
    let player = app
        .world
        .spawn()
        .insert_bundle(PlayerBundle::default())
        .id();
    let hero = app.world.spawn().insert(OwnerPlayer(player)).id();

    app.update();

    let query = app.world.query::<&PlayerHero>();
    let player_hero = query
        .get_manual(&app.world, player)
        .expect("Hero component should be added to the player");
    assert_eq!(
        player_hero.0, hero,
        "Hero component should reference the spawned hero"
    );

    // Spawn another hero
    let hero = app.world.spawn().insert(OwnerPlayer(player)).id();

    app.update();

    let player_hero = query
        .get_manual(&app.world, player)
        .expect("Hero component should be on the player from the previous spawn");
    assert_eq!(
        player_hero.0, hero,
        "Hero component should update reference to the new spawned hero"
    );
}

#[test]
fn server_player_spawns_in_lobby() {
    let mut app = setup_app_in_lobby();
    app.update();

    let mut query = app.world.query_filtered::<(), With<Authority>>();
    query
        .iter(&app.world)
        .next()
        .expect("Player should be spawned"); // TODO 0.7: Use single
}

#[test]
fn server_player_spawns_with_host_command() {
    let mut app = setup_app_with_host_command();
    app.update();

    let mut query = app.world.query_filtered::<(), With<Authority>>();
    query
        .iter(&app.world)
        .next()
        .expect("Player should be spawned"); // TODO 0.7: Use single
}

fn setup_app() -> App {
    let mut app = App::new();
    app.init_resource::<Opts>()
        .add_state(AppState::InGame)
        .add_plugin(PlayerPlugin);
    app
}

fn setup_app_in_lobby() -> App {
    let mut app = App::new();
    app.init_resource::<Opts>()
        .add_state(AppState::LobbyMenu)
        .add_plugin(PlayerPlugin);
    app
}

fn setup_app_with_host_command() -> App {
    let mut app = App::new();
    app.insert_resource(Opts {
        subcommand: Some(SubCommand::Host),
    })
    .add_state(AppState::MainMenu)
    .add_plugin(PlayerPlugin);
    app
}

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

pub mod common;

use approx::assert_relative_eq;
use bevy::{
    app::Events,
    input::{mouse::MouseMotion, InputPlugin},
    prelude::*,
};
use heron::PhysicsPlugin;

use gardum::{
    characters::camera::{CameraPlugin, OrbitRotation, CAMERA_DISTANCE},
    core::{AppState, Authority},
};

#[test]
fn camera_input() {
    let mut app = setup_app();

    app.update();
    let mut events = app.world.get_resource_mut::<Events<MouseMotion>>().unwrap();
    events.send(MouseMotion { delta: Vec2::ONE });
    app.update();

    let mut query = app.world.query::<&OrbitRotation>();
    let orbit_rotation = query.iter(&mut app.world).next().unwrap();
    assert_ne!(
        *orbit_rotation,
        OrbitRotation::default(),
        "Orbital rotation should change after input"
    );
}

#[test]
fn camera_moves_around_player() {
    let mut app = setup_app();
    let player = app
        .world
        .spawn()
        .insert_bundle(DummyPlayerBundle::default())
        .id();

    app.update();
    let mut query = app.world.query_filtered::<Entity, With<OrbitRotation>>();
    let camera = query.iter(&app.world).next().unwrap();

    for _ in 1..4 {
        app.world.get_mut::<Transform>(player).unwrap().translation += Vec3::ONE * 20.0;
        app.world.get_mut::<OrbitRotation>(camera).unwrap().0 += Vec2::ONE * 120.0;

        app.update();

        let camera_transform = app.world.get::<Transform>(camera).unwrap();
        let player_transform = app.world.get::<Transform>(player).unwrap();

        assert_relative_eq!(
            camera_transform
                .translation
                .distance(player_transform.translation),
            CAMERA_DISTANCE,
            epsilon = 0.001
        );
        assert_eq!(
            *camera_transform,
            camera_transform.looking_at(player_transform.translation, Vec3::Y),
            "Camera should look at the player"
        );
    }
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(CameraPlugin);
    app_builder.app
}

#[derive(Bundle, Default)]
struct DummyPlayerBundle {
    transform: Transform,
    authority: Authority,
}

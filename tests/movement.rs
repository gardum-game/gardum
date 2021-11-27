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
    input::{keyboard::KeyboardInput, ElementState, InputPlugin},
    prelude::*,
    render::camera::Camera,
};
use heron::{CollisionShape, PhysicsPlugin, RigidBody, Velocity};

use gardum::{
    characters::movement::{MovementInput, MovementPlugin},
    core::{AppState, Authority},
};

#[test]
fn movement_input() {
    let mut app = setup_app();
    app.update();

    let test_data = [
        (
            KeyCode::W,
            MovementInput {
                forward: true,
                ..Default::default()
            },
        ),
        (
            KeyCode::S,
            MovementInput {
                backward: true,
                ..Default::default()
            },
        ),
        (
            KeyCode::A,
            MovementInput {
                left: true,
                ..Default::default()
            },
        ),
        (
            KeyCode::D,
            MovementInput {
                right: true,
                ..Default::default()
            },
        ),
    ];

    for (i, (key, expected_input)) in test_data.iter().enumerate() {
        let mut events = app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();

        if i != 0 {
            // Previous key should be released manually
            events.send(KeyboardInput {
                scan_code: 0,
                key_code: Some(test_data[i - 1].0),
                state: ElementState::Released,
            });
        }

        events.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(*key),
            state: ElementState::Pressed,
        });

        app.update();

        assert_eq!(
            app.world.get_resource::<MovementInput>().unwrap(),
            expected_input,
            "Movement input should correspond to the pressed key: {:?}",
            key
        );
    }
}

#[test]
fn player_falls() {
    let mut app = setup_app();
    app.world
        .spawn()
        .insert_bundle(DummyCameraBundle::default());
    let player = app
        .world
        .spawn()
        .insert_bundle(DummyPlayerBundle::default())
        .id();

    app.update();
    app.update();

    // TODO 0.6: Add check for floor using SystemState
    assert!(
        DummyPlayerBundle::default().transform.translation.y
            > app.world.get::<Transform>(player).unwrap().translation.y,
        "Player should be affected by gravity"
    );

    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(KeyCode::Space),
        state: ElementState::Pressed,
    });

    let previous_translation = app.world.get::<Transform>(player).unwrap().translation;

    app.update();

    assert!(
        previous_translation.y > app.world.get::<Transform>(player).unwrap().translation.y,
        "Player should't be able to jump"
    );
}

#[test]
fn player_standing_on_platform() {
    let mut app = setup_app();
    app.world
        .spawn()
        .insert_bundle(DummyCameraBundle::default());
    app.world.spawn().insert_bundle(DummyPlainBundle::default());
    let player = app
        .world
        .spawn()
        .insert_bundle(DummyPlayerBundle::default())
        .id();

    app.update();

    let previous_translation = app.world.get::<Transform>(player).unwrap().translation;

    app.update();

    // TODO 0.6: Add check for floor using SystemState
    assert_eq!(
        previous_translation.y,
        app.world.get::<Transform>(player).unwrap().translation.y,
        "Player shouldn't be affected by gravity"
    );

    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(KeyCode::Space),
        state: ElementState::Pressed,
    });

    app.update();

    assert!(
        DummyPlayerBundle::default().transform.translation.y
            < app.world.get::<Transform>(player).unwrap().translation.y,
        "Player should be able to jump"
    );
}

#[test]
fn player_moves() {
    let mut app = setup_app();
    app.world
        .spawn()
        .insert_bundle(DummyCameraBundle::default());
    let player = app
        .world
        .spawn()
        .insert_bundle(DummyPlayerBundle::default())
        .id();

    app.update();

    let test_data = [
        (KeyCode::W, -Vec3::Z),
        (KeyCode::S, Vec3::Z),
        (KeyCode::A, -Vec3::X),
        (KeyCode::D, Vec3::X),
    ];

    for (i, (key, expected_direction)) in test_data.iter().enumerate() {
        let mut events = app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();

        if i != 0 {
            // Previous key should be released manually
            events.send(KeyboardInput {
                scan_code: 0,
                key_code: Some(test_data[i - 1].0),
                state: ElementState::Released,
            });
        }

        events.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(*key),
            state: ElementState::Pressed,
        });

        let previous_translation = app
            .world
            .get::<Transform>(player)
            .unwrap()
            .translation
            .clone();

        // Clean previous velocity to avoid interpolation
        app.world.get_mut::<Velocity>(player).unwrap().linear = Vec3::ZERO;

        app.update();

        let mut direction =
            app.world.get::<Transform>(player).unwrap().translation - previous_translation;
        direction.y = 0.0; // Remove gravity
        direction = direction.normalize();

        assert_relative_eq!(direction.x, expected_direction.x);
        assert_relative_eq!(direction.y, expected_direction.y);
    }
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(MovementPlugin);
    app_builder.app
}

#[derive(Bundle)]
struct DummyPlainBundle {
    rigid_body: RigidBody,
    shape: CollisionShape,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl Default for DummyPlainBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static,
            shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(10.0, 0.1, 10.0),
                border_radius: None,
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Bundle)]
struct DummyPlayerBundle {
    rigid_body: RigidBody,
    shape: CollisionShape,
    transform: Transform,
    global_transform: GlobalTransform,
    velocity: Velocity,
    authority: Authority,
}

impl Default for DummyPlayerBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::Capsule {
                half_segment: 0.5,
                radius: 0.5,
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            velocity: Velocity::default(),
            authority: Authority,
        }
    }
}

#[derive(Bundle, Default)]
struct DummyCameraBundle {
    camera: Camera,
    transform: Transform,
    authority: Authority,
}

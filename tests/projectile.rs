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

use bevy::{app::Events, prelude::*};
use heron::{CollisionShape, PhysicsPlugin, RigidBody, Velocity};

use gardum::{
    characters::projectile::{ProjectileBundle, ProjectileHitEvent, ProjectilePlugin},
    core::AppState,
};

#[test]
fn projectile_moves() {
    let mut app = setup_app();
    let projectile_entity = app
        .world
        .spawn()
        .insert_bundle(ProjectileBundle {
            velocity: Velocity::from_linear(Vec3::ONE),
            ..Default::default()
        })
        .id();

    app.update();
    app.update();

    let transform = app.world.get::<Transform>(projectile_entity).unwrap();
    assert!(
        transform.translation.length() > 0.0,
        "Projectile should be moved by velocity"
    );
}

#[test]
fn projectiles_do_not_collide() {
    let mut app = setup_app();
    app.world.spawn().insert_bundle(ProjectileBundle::default());
    app.world.spawn().insert_bundle(ProjectileBundle::default());

    app.update();
    app.update();

    assert_eq!(
        app.world.entities().len(),
        2,
        "Projectiles shouldn't be destroyed when colliding with each other"
    );

    let events = app
        .world
        .get_resource::<Events<ProjectileHitEvent>>()
        .unwrap();
    let mut reader = events.get_reader();

    assert_eq!(
        reader.iter(&events).count(),
        0,
        "Hit event shouldn't be triggered for collision between two projectiles"
    );
}

#[test]
fn objects_do_not_collide() {
    let mut app = setup_app();
    // Spawn in different order
    app.world.spawn().insert_bundle(DummyBundle::default());
    app.world.spawn().insert_bundle(DummyBundle::default());

    app.update();
    app.update();

    assert_eq!(
        app.world.entities().len(),
        2,
        "Objects shouldn't be destroyed when colliding with each other"
    );

    let events = app
        .world
        .get_resource::<Events<ProjectileHitEvent>>()
        .unwrap();
    let mut reader = events.get_reader();

    assert_eq!(
        reader.iter(&events).count(),
        0,
        "Hit event shouldn't be triggered for collision between two objects"
    );
}

#[test]
fn projectile_collides_with_object() {
    let mut app = setup_app();
    let projectile = app
        .world
        .spawn()
        .insert_bundle(ProjectileBundle::default())
        .id();
    let object = app.world.spawn().insert_bundle(DummyBundle::default()).id();

    app.update();
    app.update();

    assert_eq!(
        app.world.entities().len(),
        1,
        "Projectiles are destroyed when colliding with other objects"
    );

    let events = app
        .world
        .get_resource::<Events<ProjectileHitEvent>>()
        .unwrap();
    let mut reader = events.get_reader();
    let event = reader
        .iter(&events)
        .next()
        .expect("Hit event should be triggered");

    assert_eq!(
        event.projectile, projectile,
        "Hit event should have the same projectile"
    );
    assert_eq!(
        event.target, object,
        "Hit event should have the same target"
    );
    assert!(
        reader.iter(&events).next().is_none(),
        "There should be no more collision events"
    );
}

#[test]
fn object_collides_with_projectile() {
    let mut app = setup_app();
    // Spawn in different order
    let object = app.world.spawn().insert_bundle(DummyBundle::default()).id();
    let projectile = app
        .world
        .spawn()
        .insert_bundle(ProjectileBundle::default())
        .id();

    app.update();
    app.update();

    assert_eq!(
        app.world.entities().len(),
        1,
        "Projectiles are destroyed when colliding with other objects"
    );

    let events = app
        .world
        .get_resource::<Events<ProjectileHitEvent>>()
        .unwrap();
    let mut reader = events.get_reader();
    let event = reader
        .iter(&events)
        .next()
        .expect("Hit event should be triggered");

    assert_eq!(
        event.projectile, projectile,
        "Hit event should have the same projectile"
    );
    assert_eq!(
        event.target, object,
        "Hit event should have the same target"
    );
    assert!(
        reader.iter(&events).next().is_none(),
        "There should be no more collision events"
    );
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ProjectilePlugin);
    app_builder.app
}

#[derive(Bundle)]
struct DummyBundle {
    rigid_body: RigidBody,
    shape: CollisionShape,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl Default for DummyBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static,
            shape: CollisionShape::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

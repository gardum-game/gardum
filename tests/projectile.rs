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

use bevy::app::Events;
use bevy::prelude::*;
use heron::{CollisionShape, PhysicsPlugin, RigidBody, Velocity};

use gardum::{
    characters::projectile::{ProjectileBundle, ProjectileHitEvent, ProjectilePlugin},
    core::AppState,
};

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ProjectilePlugin);
    app_builder.app
}

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
        "Should be moved by velocity"
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
        "Projectiles do not collide with each other"
    );
}

#[test]
fn projectile_collides() {
    let mut app = setup_app();
    app.world.spawn().insert_bundle(ProjectileBundle::default());
    app.world
        .spawn()
        .insert_bundle(PbrBundle::default())
        .insert(RigidBody::Static)
        .insert(CollisionShape::default());

    app.update();
    app.update();

    assert_eq!(
        app.world.entities().len(),
        1,
        "Projectiles are destroyed when colliding with other objects"
    );

    let events = app
        .world
        .get_resource_mut::<Events<ProjectileHitEvent>>()
        .unwrap();

    let mut reader = events.get_reader();
    assert_eq!(
        reader.iter(&events).count(),
        1,
        "One hit event should be triggered"
    );
}

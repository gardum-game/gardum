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

mod common;

use approx::assert_relative_eq;
use bevy::{app::Events, prelude::*, render::camera::Camera};

use common::HeadlessRenderPlugin;
use gardum::{
    characters::{
        ability::ActivationEvent,
        health::DamageEvent,
        heroes::north::{
            FrostBoltBundle, FrostBoltProjectile, NorthPlugin, FROST_BOLT_DAMAGE,
            FROST_BOLT_SPAWN_OFFSET,
        },
        projectile::{Projectile, ProjectileHitEvent},
        CharacterOwner,
    },
    core::AppState,
};

#[test]
fn frost_bolt() {
    let mut app = setup_app();
    let ability = app
        .world
        .spawn()
        .insert_bundle(FrostBoltBundle::default())
        .id();
    let caster = app
        .world
        .spawn()
        .insert(Transform::from_translation(Vec3::ONE))
        .id();
    app.world
        .spawn()
        .insert_bundle(DummyCameraBundle::default())
        .id();

    let mut events = app
        .world
        .get_resource_mut::<Events<ActivationEvent>>()
        .unwrap();

    events.send(ActivationEvent { caster, ability });

    app.update();
    app.update();

    let mut caster_query = app.world.query_filtered::<&Transform, Without<Camera>>();
    let mut projectile_query = app.world.query_filtered::<&Transform, With<Projectile>>();
    let mut camera_query = app.world.query_filtered::<&Transform, With<Camera>>();

    let caster_transform = caster_query.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
    let projectile_transform = projectile_query.iter(&app.world).next().unwrap(); // TODO 0.7: Use single

    assert_relative_eq!(
        caster_transform.translation.x,
        projectile_transform.translation.x
    );
    assert_relative_eq!(
        caster_transform.translation.y + FROST_BOLT_SPAWN_OFFSET,
        projectile_transform.translation.y
    );
    assert_relative_eq!(
        caster_transform.translation.z,
        projectile_transform.translation.z
    );
    assert_eq!(
        caster_transform.scale, projectile_transform.scale,
        "Spawned projectile must be of the same scale as the caster"
    );

    let camera_trasnform = camera_query.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
    assert_eq!(
        projectile_transform.rotation,
        camera_trasnform.rotation * Quat::from_rotation_x(90.0_f32.to_radians()),
        "Spawned projectile must be turned towards the camera."
    );
}

#[test]
fn frost_bolt_hit() {
    let mut app = setup_app();
    let instigator = app.world.spawn().id();
    let projectile = app
        .world
        .spawn()
        .insert(FrostBoltProjectile)
        .insert(CharacterOwner(instigator))
        .id();
    let target = app.world.spawn().id();

    let mut events = app
        .world
        .get_resource_mut::<Events<ProjectileHitEvent>>()
        .unwrap();

    events.send(ProjectileHitEvent { projectile, target });

    app.update();

    let events = app.world.get_resource::<Events<DamageEvent>>().unwrap();
    let mut reader = events.get_reader();
    let event = reader.iter(&events).next().unwrap();

    assert_eq!(
        event.instigator, instigator,
        "Instigator should be equal to specified"
    );
    assert_eq!(event.target, target, "Target should be equal to specified");
    assert_eq!(
        event.damage, FROST_BOLT_DAMAGE,
        "Damage should be equal to frost bolt damage"
    );
}

fn setup_app() -> App {
    let mut app = App::new();
    app.add_event::<ActivationEvent>()
        .add_event::<ProjectileHitEvent>()
        .add_event::<DamageEvent>()
        .add_state(AppState::InGame)
        .add_plugin(HeadlessRenderPlugin)
        .add_plugin(NorthPlugin);

    app
}

#[derive(Bundle)]
struct DummyCameraBundle {
    transform: Transform,
    camera: Camera,
}

impl Default for DummyCameraBundle {
    fn default() -> Self {
        Self {
            transform: Transform::from_rotation(Quat::from_rotation_x(90_f32.to_radians())),
            camera: Camera::default(),
        }
    }
}

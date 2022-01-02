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

use bevy::{app::Events, ecs::system::CommandQueue, prelude::*};

use gardum::{
    characters::{
        ability::ActivationEvent,
        health::{DamageEvent, HealEvent},
        heroes::{HeroKind, HeroSpawnEvent, HeroesPlugin},
        projectile::ProjectileHitEvent,
    },
    core::{player::PlayerOwner, AppState, Authority},
};

use strum::IntoEnumIterator;

#[test]
fn hero_spawns_with_authority() {
    let mut app = setup_app();
    let player = app.world.spawn().insert(Authority).id();

    let mut events = app
        .world
        .get_resource_mut::<Events<HeroSpawnEvent>>()
        .unwrap();

    events.send(HeroSpawnEvent {
        player,
        kind: HeroKind::iter().next().unwrap(),
        transform: Transform::default(),
    });

    app.update();

    let mut query = app
        .world
        .query_filtered::<&PlayerOwner, (With<Authority>, With<HeroKind>)>();
    let assigned_player = query
        .iter(&app.world)
        .next()
        .expect("Hero should be spawned with authority and assigned player"); // TODO 0.6: Use single
    assert_eq!(
        assigned_player.0, player,
        "Assigned player should be equal to specified"
    )
}

#[test]
fn hero_spawns_without_authority() {
    let mut app = setup_app();
    let player = app.world.spawn().id();

    let mut events = app
        .world
        .get_resource_mut::<Events<HeroSpawnEvent>>()
        .unwrap();

    events.send(HeroSpawnEvent {
        player,
        kind: HeroKind::iter().next().unwrap(),
        transform: Transform::default(),
    });

    app.update();

    let mut query = app
        .world
        .query_filtered::<&PlayerOwner, (Without<Authority>, With<HeroKind>)>();
    let assigned_player = query
        .iter(&app.world)
        .next()
        .expect("Hero should be spawned with authority and assigned player"); // TODO 0.6: Use single
    assert_eq!(
        assigned_player.0, player,
        "Assigned player should be equal to specified"
    )
}

#[test]
fn hero_spawns_at_position() {
    let mut app = setup_app();
    let player = app.world.spawn().id();

    for expected_translation in [Vec3::ZERO, Vec3::ONE] {
        let mut events = app
            .world
            .get_resource_mut::<Events<HeroSpawnEvent>>()
            .unwrap();

        events.send(HeroSpawnEvent {
            player,
            kind: HeroKind::iter().next().unwrap(),
            transform: Transform::from_translation(expected_translation),
        });

        app.update();

        let mut query = app
            .world
            .query_filtered::<(Entity, &Transform), With<HeroKind>>();
        let (kind, transform) = query
            .iter(&app.world)
            .next()
            .expect("Hero should be spawned");
        assert_eq!(
            transform.translation, expected_translation,
            "Hero should be spawned with the specified translation"
        );

        // TODO 0.6: Use world.clear_entities()
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        commands.entity(kind).despawn_recursive();
        queue.apply(&mut app.world);
    }
}

#[test]
fn hero_spawns_with_kind() {
    let mut app = setup_app();
    let player = app.world.spawn().id();

    for expected_kind in HeroKind::iter() {
        let mut events = app
            .world
            .get_resource_mut::<Events<HeroSpawnEvent>>()
            .unwrap();

        events.send(HeroSpawnEvent {
            player,
            kind: expected_kind,
            transform: Transform::default(),
        });

        app.update();

        let mut query = app.world.query::<(Entity, &HeroKind)>();

        let (hero, kind) = query
            .iter(&app.world)
            .next()
            .expect("Hero should be spawned");
        assert_eq!(*kind, expected_kind, "The specified hero should be spawned");

        // TODO 0.6: Use world.clear_entities()
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        commands.entity(hero).despawn_recursive();
        queue.apply(&mut app.world);
    }
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_event::<ActivationEvent>()
        .add_event::<ProjectileHitEvent>()
        .add_event::<DamageEvent>()
        .add_event::<HealEvent>()
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(HeroesPlugin);

    app_builder.app
}

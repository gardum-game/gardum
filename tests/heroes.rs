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

use bevy::{app::Events, prelude::*};
use strum::IntoEnumIterator;

use common::HeadlessRenderPlugin;
use gardum::{
    characters::{
        ability::ActivationEvent,
        health::{DamageEvent, HealEvent},
        heroes::{HeroKind, HeroSelectEvent, HeroesPlugin, OwnerPlayer},
        projectile::ProjectileHitEvent,
    },
    core::{player::PlayerHero, AppState, Authority},
};

#[test]
fn old_hero_despawns() {
    let mut app = setup_app();
    let old_hero = app
        .world
        .spawn()
        .insert(HeroKind::iter().next().unwrap())
        .id();
    let player = app.world.spawn().insert(PlayerHero(old_hero)).id();

    let mut events = app
        .world
        .get_resource_mut::<Events<HeroSelectEvent>>()
        .unwrap();

    events.send(HeroSelectEvent {
        player,
        kind: HeroKind::iter().next().unwrap(),
        transform: Transform::default(),
    });

    app.update();

    let mut query = app.world.query_filtered::<Entity, With<HeroKind>>();
    let hero = query
        .iter(&app.world)
        .next()
        .expect("Should be a single hero"); // TODO 0.7: Use single
    assert_ne!(old_hero, hero, "New hero should replace the old one")
}

#[test]
fn hero_spawns_with_authority() {
    let mut app = setup_app();
    let player = app.world.spawn().insert(Authority).id();

    let mut events = app
        .world
        .get_resource_mut::<Events<HeroSelectEvent>>()
        .unwrap();

    events.send(HeroSelectEvent {
        player,
        kind: HeroKind::iter().next().unwrap(),
        transform: Transform::default(),
    });

    app.update();

    let mut query = app
        .world
        .query_filtered::<&OwnerPlayer, (With<Authority>, With<HeroKind>)>();
    let assigned_player = query
        .iter(&app.world)
        .next()
        .expect("Hero should be spawned with authority and assigned player"); // TODO 0.7: Use single
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
        .get_resource_mut::<Events<HeroSelectEvent>>()
        .unwrap();

    events.send(HeroSelectEvent {
        player,
        kind: HeroKind::iter().next().unwrap(),
        transform: Transform::default(),
    });

    app.update();

    let mut query = app
        .world
        .query_filtered::<&OwnerPlayer, (Without<Authority>, With<HeroKind>)>();
    let assigned_player = query
        .iter(&app.world)
        .next()
        .expect("Hero should be spawned with authority and assigned player"); // TODO 0.7: Use single
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
            .get_resource_mut::<Events<HeroSelectEvent>>()
            .unwrap();

        events.send(HeroSelectEvent {
            player,
            kind: HeroKind::iter().next().unwrap(),
            transform: Transform::from_translation(expected_translation),
        });

        app.update();

        let mut query = app
            .world
            .query_filtered::<(Entity, &Transform), With<HeroKind>>();
        let (hero, transform) = query
            .iter(&app.world)
            .next()
            .expect("Hero should be spawned"); // TODO 0.7: Use single
        assert_eq!(
            transform.translation, expected_translation,
            "Hero should be spawned with the specified translation"
        );

        app.world.entity_mut(hero).despawn();
    }
}

#[test]
fn hero_spawns_with_kind() {
    let mut app = setup_app();
    let player = app.world.spawn().id();

    for expected_kind in HeroKind::iter() {
        let mut events = app
            .world
            .get_resource_mut::<Events<HeroSelectEvent>>()
            .unwrap();

        events.send(HeroSelectEvent {
            player,
            kind: expected_kind,
            transform: Transform::default(),
        });

        app.update();

        let mut query = app.world.query::<(Entity, &HeroKind)>();

        let (hero, kind) = query
            .iter(&app.world)
            .next()
            .expect("Hero should be spawned"); // TODO 0.7: Use single
        assert_eq!(*kind, expected_kind, "The specified hero should be spawned");

        app.world.entity_mut(hero).despawn();
    }
}

fn setup_app() -> App {
    let mut app = App::new();
    app.add_event::<ActivationEvent>()
        .add_event::<ProjectileHitEvent>()
        .add_event::<DamageEvent>()
        .add_event::<HealEvent>()
        .add_state(AppState::InGame)
        .add_plugin(HeadlessRenderPlugin)
        .add_plugin(HeroesPlugin);

    app
}

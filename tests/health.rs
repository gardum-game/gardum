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

use bevy::{app::Events, prelude::*};

use gardum::{
    characters::health::{DamageEvent, HealEvent, Health, HealthPlugin},
    core::{AppState, Damage, Healing, PlayerBundle},
};

#[test]
fn healing() {
    let mut app = setup_app();
    let target = app.world.spawn().insert(Health::default()).id();
    app.world
        .spawn()
        .insert_bundle(PlayerBundle::default())
        .push_children(&[target])
        .id();

    let instigator = app.world.spawn().id();
    let instigator_player = app
        .world
        .spawn()
        .insert_bundle(PlayerBundle::default())
        .push_children(&[instigator])
        .id();

    for (initial_health, heal, expected_healing, expected_health) in [
        (90, 5, 5, 95),
        (90, 20, 10, Health::default().max),
        (90, 10, 10, Health::default().max),
        (0, 20, 0, 0),
    ] {
        app.world.get_mut::<Health>(target).unwrap().current = initial_health;
        app.world.get_mut::<Healing>(instigator_player).unwrap().0 = 0;

        let mut events = app.world.get_resource_mut::<Events<HealEvent>>().unwrap();
        events.send(HealEvent {
            instigator,
            target,
            heal,
        });

        app.update();

        let health = app.world.get::<Health>(target).unwrap();
        assert_eq!(
            health.current, expected_health,
            "Healing from {} for {} points should set health to {}",
            initial_health, heal, expected_health
        );

        let healing = app.world.get::<Healing>(instigator_player).unwrap();
        assert_eq!(
            healing.0, expected_healing,
            "Healing from {} for {} points should set amount of healing to {}",
            initial_health, heal, expected_healing
        );
    }
}

#[test]
fn damaging() {
    let mut app = setup_app();
    let target = app.world.spawn().insert(Health::default()).id();
    app.world
        .spawn()
        .insert_bundle(PlayerBundle::default())
        .push_children(&[target])
        .id();

    let instigator = app.world.spawn().id();
    let instigator_player = app
        .world
        .spawn()
        .insert_bundle(PlayerBundle::default())
        .push_children(&[instigator])
        .id();

    for (initial_health, damage, expected_damage, expected_health) in [
        (90, 5, 5, 85),
        (90, 95, 90, 0),
        (90, 90, 90, 0),
        (0, 20, 0, 0),
    ] {
        app.world.get_mut::<Health>(target).unwrap().current = initial_health;
        app.world.get_mut::<Damage>(instigator_player).unwrap().0 = 0;

        let mut events = app.world.get_resource_mut::<Events<DamageEvent>>().unwrap();
        events.send(DamageEvent {
            instigator,
            target,
            damage,
        });

        app.update();

        let health = app.world.get::<Health>(target).unwrap();
        assert_eq!(
            health.current, expected_health,
            "Damaging from {} for {} points should set health to {}",
            initial_health, damage, expected_health
        );

        let damaging = app.world.get::<Damage>(instigator_player).unwrap();
        assert_eq!(
            damaging.0, expected_damage,
            "Damaging from {} for {} points should set amount of damage to {}",
            initial_health, damage, expected_damage
        );
    }
}

#[test]
fn self_damaging() {
    let damage: usize = Health::default().max / 2;

    let mut app = setup_app();
    let target = app.world.spawn().insert(Health::default()).id();
    let target_player = app
        .world
        .spawn()
        .insert_bundle(PlayerBundle::default())
        .push_children(&[target])
        .id();

    let mut events = app.world.get_resource_mut::<Events<DamageEvent>>().unwrap();
    events.send(DamageEvent {
        instigator: target,
        target,
        damage,
    });

    app.update();

    let health = app.world.get::<Health>(target).unwrap();
    assert_eq!(
        health.current,
        Health::default().current - damage,
        "Health should decrease by the amount of damage"
    );

    let healing = app.world.get::<Damage>(target_player).unwrap();
    assert_eq!(
        healing.0, 0,
        "Amount of damage shouldn't increase for self-damage"
    );
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(HealthPlugin);

    app_builder.app
}

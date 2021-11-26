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
use test_case::test_case;

use gardum::{
    characters::health::{DamageEvent, HealEvent, Health, HealthPlugin},
    core::{AppState, Damage, Healing, PlayerBundle},
};

const MAX_HEALTH: usize = 100;

#[test_case(90, 5, 5, 95; "With underflow")]
#[test_case(90, 20, 10, 100; "With overflow")]
#[test_case(90, 10, 10, 100; "To full health")]
#[test_case(0, 20, 0, 0; "Dead immune to healing")]
fn healing(current_health: usize, heal: usize, expected_healing: usize, expected_health: usize) {
    let mut app = setup_app();
    let target = app
        .world
        .spawn()
        .insert(Health {
            current: current_health,
            max: MAX_HEALTH,
        })
        .id();
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

    let mut events = app.world.get_resource_mut::<Events<HealEvent>>().unwrap();
    events.send(HealEvent {
        instigator,
        target,
        heal,
    });

    app.update();

    let health = app.world.get::<Health>(target).unwrap();
    assert_eq!(health.current, expected_health);

    let healing = app.world.get::<Healing>(instigator_player).unwrap();
    assert_eq!(healing.0, expected_healing);
}

#[test_case(90, 5, 5, 85; "With underflow")]
#[test_case(90, 95, 90, 0; "With overflow")]
#[test_case(90, 90, 90, 0; "To death")]
#[test_case(0, 20, 0, 0; "Dead immune to damage")]
fn damaging(current_health: usize, damage: usize, expected_damage: usize, expected_health: usize) {
    let mut app = setup_app();
    let target = app
        .world
        .spawn()
        .insert(Health {
            current: current_health,
            max: MAX_HEALTH,
        })
        .id();
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

    let mut events = app.world.get_resource_mut::<Events<DamageEvent>>().unwrap();
    events.send(DamageEvent {
        instigator,
        target,
        damage,
    });

    app.update();

    let health = app.world.get::<Health>(target).unwrap();
    assert_eq!(health.current, expected_health);

    let damage = app.world.get::<Damage>(instigator_player).unwrap();
    assert_eq!(damage.0, expected_damage);
}

#[test]
fn self_damaging() {
    const CURRENT_HEALTH: usize = MAX_HEALTH / 2;
    const DAMAGE: usize = CURRENT_HEALTH / 2;

    let mut app = setup_app();
    let target = app
        .world
        .spawn()
        .insert(Health {
            current: CURRENT_HEALTH,
            max: MAX_HEALTH,
        })
        .id();
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
        damage: DAMAGE,
    });

    app.update();

    let health = app.world.get::<Health>(target).unwrap();
    assert_eq!(
        health.current,
        CURRENT_HEALTH - DAMAGE,
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

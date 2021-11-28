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

use bevy::{
    app::Events,
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState, InputPlugin},
    prelude::*,
};

use common::events_count;
use gardum::{
    characters::{
        ability::{Abilities, AbilityPlugin, AbilitySlot, ActivationEvent},
        cooldown::{Cooldown, CooldownPlugin},
    },
    core::{AppState, Authority},
};

#[test]
fn ability_input() {
    let mut app = setup_app();

    // No Inputs
    assert_eq!(
        *app.world.get_resource::<Option<AbilitySlot>>().unwrap(),
        None
    );

    let inputs = [
        (KeyCode::Q, AbilitySlot::Ability1),
        (KeyCode::E, AbilitySlot::Ability2),
        (KeyCode::LShift, AbilitySlot::Ability3),
        (KeyCode::R, AbilitySlot::Ultimate),
    ];
    for (key, ability_slot) in inputs {
        let mut events = app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();

        events.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(key),
            state: ElementState::Pressed,
        });

        app.update();

        assert_eq!(
            *app.world.get_resource::<Option<AbilitySlot>>().unwrap(),
            Some(ability_slot),
            "Ability slot shoud correspond to the pressed key"
        );
    }

    let mut events = app
        .world
        .get_resource_mut::<Events<MouseButtonInput>>()
        .unwrap();

    events.send(MouseButtonInput {
        button: MouseButton::Left,
        state: ElementState::Pressed,
    });

    app.update();

    assert_eq!(
        *app.world.get_resource::<Option<AbilitySlot>>().unwrap(),
        Some(AbilitySlot::BaseAttack)
    );

    // Check if input was cleared
    app.update();

    assert_eq!(
        *app.world.get_resource::<Option<AbilitySlot>>().unwrap(),
        None
    );
}

#[test]
fn ability_ignores_unrelated_action() {
    let mut app = setup_app();
    let ability = app
        .world
        .spawn()
        .insert_bundle(DummyAbilityBundle::default())
        .id();
    app.world
        .spawn()
        .insert_bundle(DummyCasterBundle::new(ability))
        .id();

    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(KeyCode::E),
        state: ElementState::Pressed,
    });

    app.update();

    assert_eq!(
        events_count::<ActivationEvent>(&mut app.world),
        0,
        "Ability shouldn't be activated because of different key"
    );
}

#[test]
fn ability_activates() {
    let mut app = setup_app();
    let ability = app
        .world
        .spawn()
        .insert_bundle(DummyAbilityBundle::default())
        .id();
    app.world
        .spawn()
        .insert_bundle(DummyCasterBundle::new(ability))
        .id();

    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(KeyCode::Q),
        state: ElementState::Pressed,
    });

    app.update();

    assert_eq!(
        events_count::<ActivationEvent>(&mut app.world),
        1,
        "Ability should be activated"
    );

    let cooldown = app.world.get::<Cooldown>(ability).unwrap();
    assert!(!cooldown.finished(), "Cooldown should be triggered");
}

#[test]
fn ability_affected_by_cooldown() {
    let mut app = setup_app();
    let ability = app
        .world
        .spawn()
        .insert_bundle(DummyAbilityBundle::default())
        .id();
    app.world
        .spawn()
        .insert_bundle(DummyCasterBundle::new(ability))
        .id();

    let mut cooldown = app.world.get_mut::<Cooldown>(ability).unwrap();
    cooldown.reset();

    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(KeyCode::Q),
        state: ElementState::Pressed,
    });

    app.update();

    assert_eq!(
        events_count::<ActivationEvent>(&mut app.world),
        0,
        "Ability shouldn't be activated because of cooldown"
    );
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(CooldownPlugin)
        .add_plugin(AbilityPlugin);
    app_builder.app
}

#[derive(Bundle)]
struct DummyCasterBundle {
    authority: Authority,
    abilities: Abilities,
}

impl DummyCasterBundle {
    fn new(dummy_ability: Entity) -> Self {
        Self {
            authority: Authority,
            abilities: Abilities(vec![dummy_ability]),
        }
    }
}

#[derive(Bundle)]
struct DummyAbilityBundle {
    slot: AbilitySlot,
    cooldown: Cooldown,
}

impl Default for DummyAbilityBundle {
    fn default() -> Self {
        Self {
            slot: AbilitySlot::Ability1,
            cooldown: Cooldown::from_secs(1),
        }
    }
}

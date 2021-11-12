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

use bevy::{
    app::Events,
    ecs::system::CommandQueue,
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState, InputPlugin},
    prelude::*,
};

use gardum::{
    characters::ability::{Abilities, AbilityPlugin, AbilitySlot, ActivationEvent, Cooldown},
    core::{AppState, Authority},
};

#[test]
fn ability_input() {
    let mut app = setup_app();

    assert_ability_input(&mut app, None);

    simulate_key_press(&mut app, KeyCode::Q);
    assert_ability_input(&mut app, Some(AbilitySlot::Ability1));

    simulate_key_press(&mut app, KeyCode::E);
    assert_ability_input(&mut app, Some(AbilitySlot::Ability2));

    simulate_key_press(&mut app, KeyCode::LShift);
    assert_ability_input(&mut app, Some(AbilitySlot::Ability3));

    simulate_key_press(&mut app, KeyCode::R);
    assert_ability_input(&mut app, Some(AbilitySlot::Ultimate));

    simulate_mouse_press(&mut app, MouseButton::Left);
    assert_ability_input(&mut app, Some(AbilitySlot::BaseAttack));

    // Check if input was cleared
    app.update();
    assert_ability_input(&mut app, None);
}

fn assert_ability_input(app: &mut App, slot: Option<AbilitySlot>) {
    let activated_ability = app.world.get_resource::<Option<AbilitySlot>>().unwrap();
    assert_eq!(*activated_ability, slot);
}

#[test]
fn ability_activation_and_destruction() {
    let mut app = setup_app();

    let ability = app
        .world
        .spawn()
        .insert_bundle(DummyAbilityBundle::default())
        .id();
    let caster = app
        .world
        .spawn()
        .insert(Abilities(vec![ability]))
        .insert(Authority)
        .id();

    simulate_key_press(&mut app, KeyCode::E);
    app.update();

    let events = app.world.get_resource::<Events<ActivationEvent>>().unwrap();

    let mut reader = events.get_reader();
    assert_eq!(
        reader.iter(&events).count(),
        0,
        "Ability shouldn't be activated because of different key"
    );

    simulate_key_press(&mut app, KeyCode::Q);
    app.update();

    let events = app.world.get_resource::<Events<ActivationEvent>>().unwrap();

    assert_eq!(
        reader.iter(&events).count(),
        1,
        "Ability should be activated"
    );

    simulate_key_press(&mut app, KeyCode::Q);
    app.update();

    let events = app.world.get_resource::<Events<ActivationEvent>>().unwrap();

    assert_eq!(
        reader.iter(&events).count(),
        0,
        "Ability shouldn't be activated because of cooldown"
    );

    // TODO 0.6: Use world.entity_mut
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &app.world);
    commands.entity(caster).despawn_recursive();
    queue.apply(&mut app.world);
    assert_eq!(
        app.world.entities().len(),
        0,
        "Entities of abilities must be destroyed along with the owner"
    );
}

fn simulate_key_press(app: &mut App, code: KeyCode) {
    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(code),
        state: ElementState::Pressed,
    });

    app.update();
}

fn simulate_mouse_press(app: &mut App, button: MouseButton) {
    let mut events = app
        .world
        .get_resource_mut::<Events<MouseButtonInput>>()
        .unwrap();

    events.send(MouseButtonInput {
        button,
        state: ElementState::Pressed,
    });

    app.update();
}

fn setup_app() -> App {
    let mut app_builder = App::build();
    app_builder
        .add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(AbilityPlugin);
    app_builder.app
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
            cooldown: Cooldown::from_secs(4),
        }
    }
}

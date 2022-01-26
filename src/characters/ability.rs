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

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};

use super::{cooldown::Cooldown, CharacterControl};
use crate::core::AppState;

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Option<AbilitySlot>>()
            .add_event::<ActivationEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label(AbilitySystems::InputSet)
                    .with_system(input_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after(AbilitySystems::InputSet)
                    .with_system(activation_system),
            );
    }
}

fn input_system(
    character_control: Option<Res<CharacterControl>>,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut input: ResMut<Option<AbilitySlot>>,
) {
    if character_control.is_none() {
        return;
    }

    if keys.just_pressed(KeyCode::Q) {
        *input = Some(AbilitySlot::Ability1);
        return;
    }

    if keys.just_pressed(KeyCode::E) {
        *input = Some(AbilitySlot::Ability2);
        return;
    }

    if keys.just_pressed(KeyCode::LShift) {
        *input = Some(AbilitySlot::Ability3);
        return;
    }

    if keys.just_pressed(KeyCode::R) {
        *input = Some(AbilitySlot::Ultimate);
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        *input = Some(AbilitySlot::BaseAttack);
        return;
    }

    *input = None;
}

fn activation_system(
    activated_slot: Res<Option<AbilitySlot>>,
    mut events: EventWriter<ActivationEvent>,
    characters: Query<(Entity, &Abilities)>,
    mut abilities: Query<(&AbilitySlot, Option<&mut Cooldown>)>,
) {
    let input = match *activated_slot {
        Some(input) => input,
        None => return,
    };

    for (character, character_abilities) in characters.iter() {
        for ability in character_abilities.iter() {
            let (slot, cooldown) = abilities.get_mut(*ability).unwrap();

            if input != *slot {
                continue;
            }

            if let Some(mut cooldown) = cooldown {
                if !cooldown.finished() {
                    return;
                }
                cooldown.reset();
            }

            events.send(ActivationEvent {
                character,
                ability: *ability,
            });
            return;
        }
    }
}

pub(super) struct ActivationEvent {
    pub(super) character: Entity,
    pub(super) ability: Entity,
}

#[derive(Copy, Clone, Component, PartialEq, Debug)]
pub(super) enum AbilitySlot {
    BaseAttack,
    Ability1,
    Ability2,
    Ability3,
    Ultimate,
}

#[derive(Default, Deref, DerefMut, Component)]
pub(crate) struct Abilities(pub(crate) Vec<Entity>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum AbilitySystems {
    InputSet,
}

#[cfg(test)]
mod tests {
    use bevy::{
        app::Events,
        input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState, InputPlugin},
    };

    use super::*;
    use crate::{characters::cooldown::CooldownPlugin, core::Local};

    #[test]
    fn ability_input() {
        let mut app = setup_app();

        // No Inputs
        assert_eq!(
            *app.world.get_resource::<Option<AbilitySlot>>().unwrap(),
            None
        );

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
            *app.world.get_resource::<Option<AbilitySlot>>().unwrap(),
            None,
            "Ability slot shouldn't exist without character control"
        );

        let mut events = app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();

        events.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(KeyCode::Q),
            state: ElementState::Released,
        });

        app.insert_resource(CharacterControl);

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
            .insert_bundle(DummyCharacterBundle::new(ability))
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

        let events = app.world.get_resource::<Events<ActivationEvent>>().unwrap();
        let mut reader = events.get_reader();

        assert_eq!(
            reader.iter(&events).count(),
            0,
            "Activation event shouldn't be triggered for unrelated key"
        );
    }

    #[test]
    fn ability_activates() {
        let mut app = setup_app();
        app.insert_resource(CharacterControl);
        let ability = app
            .world
            .spawn()
            .insert_bundle(DummyAbilityBundle::default())
            .id();
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::new(ability))
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

        let events = app.world.get_resource::<Events<ActivationEvent>>().unwrap();
        let mut reader = events.get_reader();
        let event = reader
            .iter(&events)
            .next()
            .expect("Activation event should be triggered");

        assert_eq!(
            event.character, character,
            "Activation event should have the same character"
        );
        assert_eq!(
            event.ability, ability,
            "Activation event should have the same ability"
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
            .insert_bundle(DummyCharacterBundle::new(ability))
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

        let events = app.world.get_resource::<Events<ActivationEvent>>().unwrap();
        let mut reader = events.get_reader();

        assert_eq!(
            reader.iter(&events).count(),
            0,
            "Activation event shouldn't be triggered because of cooldown"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(InputPlugin)
            .add_plugin(CooldownPlugin)
            .add_plugin(AbilityPlugin);
        app
    }

    #[derive(Bundle)]
    struct DummyCharacterBundle {
        local: Local,
        abilities: Abilities,
    }

    impl DummyCharacterBundle {
        fn new(dummy_ability: Entity) -> Self {
            Self {
                local: Local,
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
}

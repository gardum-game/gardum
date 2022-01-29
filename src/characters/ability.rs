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
use leafwing_input_manager::prelude::ActionState;

use super::{character_action::CharacterAction, cooldown::Cooldown};
use crate::core::AppState;

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActivationEvent>()
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(activation_system));
    }
}

fn activation_system(
    mut events: EventWriter<ActivationEvent>,
    characters: Query<(Entity, &Abilities, &ActionState<CharacterAction>)>,
    mut abilities: Query<(&CharacterAction, Option<&mut Cooldown>)>,
) {
    for (character, character_abilities, actions) in characters.iter() {
        for ability in character_abilities.iter() {
            let (action, cooldown) = abilities.get_mut(*ability).unwrap();
            if actions.just_pressed(*action) {
                if let Some(mut cooldown) = cooldown {
                    if !cooldown.finished() {
                        break;
                    }
                    cooldown.reset();
                }

                events.send(ActivationEvent {
                    character,
                    ability: *ability,
                });
                break;
            }
        }
    }
}

pub(super) struct ActivationEvent {
    pub(super) character: Entity,
    pub(super) ability: Entity,
}

#[derive(Default, Deref, DerefMut, Component)]
pub(crate) struct Abilities(pub(crate) Vec<Entity>);

#[cfg(test)]
mod tests {
    use bevy::{app::Events, input::InputPlugin};

    use super::*;
    use crate::{characters::cooldown::CooldownPlugin, core::Local};

    #[test]
    fn ability_ignores_unrelated_action() {
        let mut app = setup_app();
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

        let mut actions = app
            .world
            .get_mut::<ActionState<CharacterAction>>(character)
            .unwrap();
        actions.press(CharacterAction::Ability2);

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

        let mut actions = app
            .world
            .get_mut::<ActionState<CharacterAction>>(character)
            .unwrap();
        actions.press(CharacterAction::Ability1);

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
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::new(ability))
            .id();

        let mut cooldown = app.world.get_mut::<Cooldown>(ability).unwrap();
        cooldown.reset();

        let mut actions = app
            .world
            .get_mut::<ActionState<CharacterAction>>(character)
            .unwrap();
        actions.press(CharacterAction::Ability1);

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
        abilities: Abilities,
        action_state: ActionState<CharacterAction>,
        local: Local,
    }

    impl DummyCharacterBundle {
        fn new(dummy_ability: Entity) -> Self {
            Self {
                abilities: Abilities(vec![dummy_ability]),
                action_state: ActionState::default(),
                local: Local,
            }
        }
    }

    #[derive(Bundle)]
    struct DummyAbilityBundle {
        action: CharacterAction,
        cooldown: Cooldown,
    }

    impl Default for DummyAbilityBundle {
        fn default() -> Self {
            Self {
                action: CharacterAction::Ability1,
                cooldown: Cooldown::from_secs(1),
            }
        }
    }
}

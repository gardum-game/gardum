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
use derive_more::{Deref, DerefMut, From};
use leafwing_input_manager::prelude::ActionState;

use super::{character_action::CharacterAction, cooldown::Cooldown, game_state::GameState};

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(activation_system));
    }
}

fn activation_system(
    mut commands: Commands,
    time: Res<Time>,
    characters: Query<(Entity, &Abilities, &ActionState<CharacterAction>)>,
    mut abilities: Query<(&CharacterAction, Option<&mut Cooldown>)>,
) {
    for (character, character_abilities, actions) in characters.iter() {
        for ability in character_abilities.iter() {
            let (action, cooldown) = abilities.get_mut(*ability).unwrap();

            if let Some(mut cooldown) = cooldown {
                cooldown.tick(time.delta());
                if actions.just_pressed(action) {
                    if !cooldown.finished() {
                        break;
                    }
                    cooldown.reset();
                }
            }

            if actions.just_pressed(action) {
                commands.entity(*ability).insert(Activator(character));
                break;
            }
        }
    }
}

/// Path to icon resource.
#[derive(Component, From)]
pub(crate) struct IconPath(pub(crate) &'static str);

/// Indicates that the ability has been activated and contains the hero that activated it
#[derive(Component)]
pub(super) struct Activator(pub(super) Entity);

#[derive(Default, Deref, DerefMut, Component)]
pub(crate) struct Abilities(pub(crate) Vec<Entity>);

#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;
    use std::time::Duration;

    use super::*;
    use crate::core::Authority;

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
        actions.press(&CharacterAction::Ability2);

        app.update();

        assert!(
            !app.world
                .get_entity(ability)
                .unwrap()
                .contains::<Activator>(),
            "Ability shouldn't be triggered for unrelated action"
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
        actions.press(&CharacterAction::Ability1);

        app.update();

        let activator = app
            .world
            .get::<Activator>(ability)
            .expect("Ability should be activated");
        assert_eq!(activator.0, character, "Character should become activator");

        let cooldown = app.world.get::<Cooldown>(ability).unwrap();
        assert!(!cooldown.finished(), "Cooldown should be triggered");
        assert_eq!(
            cooldown.elapsed(),
            Duration::default(),
            "Cooldown shouldn't have elapsed right after activation time"
        );

        app.update();

        let cooldown = app.world.get::<Cooldown>(ability).unwrap();
        assert!(
            cooldown.elapsed() > Duration::default(),
            "Cooldown should tick"
        );
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
        actions.press(&CharacterAction::Ability1);

        app.update();

        assert!(
            !app.world
                .get_entity(ability)
                .unwrap()
                .contains::<Activator>(),
            "Ability shouldn't be triggered because of cooldown"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(GameState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(InputPlugin)
            .add_plugin(AbilityPlugin);
        app
    }

    #[derive(Bundle)]
    struct DummyCharacterBundle {
        abilities: Abilities,
        action_state: ActionState<CharacterAction>,
        authority: Authority,
    }

    impl DummyCharacterBundle {
        fn new(dummy_ability: Entity) -> Self {
            Self {
                abilities: Abilities(vec![dummy_ability]),
                action_state: ActionState::default(),
                authority: Authority,
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

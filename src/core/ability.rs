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
use derive_more::From;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{control_actions::ControlAction, cooldown::Cooldown, game_state::GameState};

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::activation_system.run_in_state(GameState::InGame))
            .add_system(Self::abilities_to_children_system.run_in_state(GameState::InGame));
    }
}

impl AbilityPlugin {
    fn activation_system(
        mut commands: Commands,
        time: Res<Time>,
        characters: Query<(Entity, &Abilities, &ActionState<ControlAction>)>,
        mut abilities: Query<(&ControlAction, Option<&mut Cooldown>)>,
    ) {
        for (character, character_abilities, action_state) in characters.iter() {
            for ability in character_abilities.iter() {
                let (action, cooldown) = abilities.get_mut(*ability).unwrap();

                if let Some(mut cooldown) = cooldown {
                    cooldown.tick(time.delta());
                    if action_state.just_pressed(*action) {
                        if !cooldown.finished() {
                            break;
                        }
                        cooldown.reset();
                    }
                }

                if action_state.just_pressed(*action) {
                    commands.entity(*ability).insert(Activator(character));
                    break;
                }
            }
        }
    }

    fn abilities_to_children_system(
        mut commands: Commands,
        characters: Query<(Entity, &Abilities), Added<Abilities>>,
    ) {
        for (character, abilities) in characters.iter() {
            commands.entity(character).push_children(&abilities.0);
        }
    }
}

/// Path to icon resource.
#[derive(Component, From)]
pub(crate) struct IconPath(pub(crate) &'static str);

/// Indicates that the ability has been activated and contains the hero that activated it
#[derive(Component)]
pub(super) struct Activator(pub(super) Entity);

#[derive(Default, Deref, DerefMut, Component, From)]
pub(crate) struct Abilities(pub(crate) Vec<Entity>);

#[cfg(test)]
mod tests {
    use bevy::input::InputPlugin;
    use std::time::Duration;

    use super::*;
    use crate::core::Authority;

    #[test]
    fn ability_ignores_unrelated_action() {
        let mut app = App::new();
        app.add_plugin(TestAbilityPlugin);

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

        let mut action_state = app
            .world
            .get_mut::<ActionState<ControlAction>>(character)
            .unwrap();
        action_state.press(ControlAction::Ability2);

        app.update();

        assert!(
            !app.world.entity(ability).contains::<Activator>(),
            "Ability shouldn't be triggered for unrelated action"
        );
    }

    #[test]
    fn ability_activates() {
        let mut app = App::new();
        app.add_plugin(TestAbilityPlugin);

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

        let mut action_state = app
            .world
            .get_mut::<ActionState<ControlAction>>(character)
            .unwrap();
        action_state.press(ControlAction::Ability1);

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
        let mut app = App::new();
        app.add_plugin(TestAbilityPlugin);

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

        let mut action_state = app
            .world
            .get_mut::<ActionState<ControlAction>>(character)
            .unwrap();
        action_state.press(ControlAction::Ability1);

        app.update();

        assert!(
            !app.world.entity(ability).contains::<Activator>(),
            "Ability shouldn't be triggered because of cooldown"
        );
    }

    #[test]
    fn abilities_are_children() {
        let mut app = App::new();
        app.add_plugin(TestAbilityPlugin);

        let ability = app.world.spawn().id();
        let character = app.world.spawn().insert(Abilities(vec![ability])).id();

        app.update();

        let parent = app.world.entity(ability).get::<Parent>().unwrap();

        assert_eq!(
            character, parent.0,
            "Ability should have its character as a parent"
        );
    }

    struct TestAbilityPlugin;

    impl Plugin for TestAbilityPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(GameState::InGame)
                .add_plugins(MinimalPlugins)
                .add_plugin(InputPlugin)
                .add_plugin(AbilityPlugin);
        }
    }

    #[derive(Bundle)]
    struct DummyCharacterBundle {
        abilities: Abilities,
        action_state: ActionState<ControlAction>,
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
        action: ControlAction,
        cooldown: Cooldown,
    }

    impl Default for DummyAbilityBundle {
        fn default() -> Self {
            Self {
                action: ControlAction::Ability1,
                cooldown: Cooldown::from_secs(1),
            }
        }
    }
}

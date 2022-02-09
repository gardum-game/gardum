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
use std::time::Duration;

use super::{character_action::CharacterAction, AppState};

pub(super) struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(activation_system)
                .with_system(cooldown_system),
        );
    }
}

fn activation_system(
    mut commands: Commands,
    characters: Query<(Entity, &Abilities, &ActionState<CharacterAction>)>,
    mut abilities: Query<(&CharacterAction, Option<&mut Cooldown>)>,
) {
    for (character, character_abilities, actions) in characters.iter() {
        for ability in character_abilities.iter() {
            let (action, cooldown) = abilities.get_mut(*ability).unwrap();
            if actions.just_pressed(action) {
                if let Some(mut cooldown) = cooldown {
                    if !cooldown.finished() {
                        break;
                    }
                    cooldown.reset();
                }

                commands.entity(*ability).insert(Activator(character));
                break;
            }
        }
    }
}

fn cooldown_system(time: Res<Time>, mut cooldowns: Query<&mut Cooldown>) {
    for mut cooldown in cooldowns.iter_mut() {
        cooldown.tick(time.delta());
    }
}

#[derive(Deref, DerefMut, Component)]
pub(crate) struct Cooldown(Timer);

impl Cooldown {
    pub(super) fn from_secs(secs: u64) -> Self {
        // Setup timer in finished state
        let duration = Duration::from_secs(secs);
        let mut timer = Timer::new(duration, false);
        timer.tick(duration);

        Self(timer)
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

    use super::*;
    use crate::core::Local;

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

    #[test]
    fn cooldown_from_secs() {
        const SECONDS: u64 = 4;

        let cooldown = Cooldown::from_secs(SECONDS);
        assert_eq!(cooldown.duration(), Duration::from_secs(SECONDS));
        assert!(
            cooldown.finished(),
            "Cooldown shouldn't tick after creation"
        );
    }

    #[test]
    fn cooldown_ticks() {
        let mut app = setup_app();

        let mut cooldown = Cooldown::from_secs(1);
        cooldown.reset(); // Activate cooldown
        let cooldown_entity = app.world.spawn().insert(cooldown).id();

        app.update();
        app.update();
        let cooldown = app.world.get::<Cooldown>(cooldown_entity).unwrap();
        assert!(
            cooldown.elapsed() > Duration::default(),
            "Cooldown should tick"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(InputPlugin)
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

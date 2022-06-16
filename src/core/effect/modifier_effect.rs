/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::{
    marker::PhantomData,
    ops::{AddAssign, SubAssign},
};

use super::{Dispelled, EffectTarget};
use crate::core::game_state::GameState;

#[derive(Default)]
pub(super) struct ModifierEffectPlugin<T> {
    effect: PhantomData<T>,
}

impl<T: Component + AddAssign + SubAssign + Copy> Plugin for ModifierEffectPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(Self::apply_modifier_system.run_in_state(GameState::InGame))
            .add_system(Self::remove_modifier_system.run_in_state(GameState::InGame));
    }
}

impl<T: Component + AddAssign + SubAssign + Copy> ModifierEffectPlugin<T> {
    fn apply_modifier_system(
        added_effects: Query<(&EffectTarget, &T), Added<EffectTarget>>,
        mut characters: Query<&mut T, Without<EffectTarget>>,
    ) {
        for (target, effect_modifier) in added_effects.iter() {
            *characters.get_mut(target.0).unwrap() += *effect_modifier;
        }
    }

    fn remove_modifier_system(
        dispelled_effects: Query<(&EffectTarget, &T), Added<Dispelled>>,
        mut characters: Query<&mut T, Without<EffectTarget>>,
    ) {
        for (target, effect_modifier) in dispelled_effects.iter() {
            *characters.get_mut(target.0).unwrap() -= *effect_modifier;
        }
    }
}

#[cfg(test)]
mod tests {
    use derive_more::{AddAssign, From, SubAssign};

    use super::*;
    use crate::core::game_state::GameState;

    #[test]
    fn player_modifier_changes() {
        let mut app = App::new();
        app.add_plugin(TestModifierEffectPlugin);

        const MODIFIER_VALUE: f32 = 0.2;
        let player = app.world.spawn().insert(DummyModifier::default()).id();
        let effect = app
            .world
            .spawn()
            .insert_bundle(DummyModifierBundle {
                target: player.into(),
                modifier: MODIFIER_VALUE.into(),
            })
            .id();

        app.update();

        assert_eq!(
            app.world.entity(player).get::<DummyModifier>().unwrap().0,
            DummyModifier::default().0 + MODIFIER_VALUE,
            "Effect modifier value ({MODIFIER_VALUE}) should be added to the player's value ({value})",
            value = DummyModifier::default().0
        );
        app.world.entity_mut(effect).insert(Dispelled);

        app.update();
        assert_eq!(
            app.world.entity(player).get::<DummyModifier>().unwrap().0,
            DummyModifier::default().0,
            "Player's modifier value should be restored after effect removal"
        );
    }

    struct TestModifierEffectPlugin;

    impl Plugin for TestModifierEffectPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(GameState::InGame)
                .add_plugins(MinimalPlugins)
                .add_plugin(ModifierEffectPlugin::<DummyModifier>::default());
        }
    }

    #[derive(Bundle)]
    struct DummyModifierBundle {
        target: EffectTarget,
        modifier: DummyModifier,
    }

    #[derive(Component, Clone, Copy, SubAssign, AddAssign, From)]
    struct DummyModifier(f32);

    impl Default for DummyModifier {
        fn default() -> Self {
            Self(1.0)
        }
    }
}

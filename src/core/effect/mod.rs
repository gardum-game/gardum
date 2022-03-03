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

pub(super) mod modifier_effect;
pub(super) mod periodic_effect;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut, From};

use super::{
    character::{DamageModifier, HealingModifier, SpeedModifier},
    health::Death,
    AppState,
};
use modifier_effect::ModifierEffectPlugin;
use periodic_effect::PeriodicEffectPlugin;

pub(super) struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ModifierEffectPlugin::<SpeedModifier>::default())
            .add_plugin(ModifierEffectPlugin::<DamageModifier>::default())
            .add_plugin(ModifierEffectPlugin::<HealingModifier>::default())
            .add_plugin(PeriodicEffectPlugin)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(dispell_on_death_system)
                    .with_system(effect_timer_system)
                    .with_system(effect_removal_system),
            );
    }
}

fn effect_removal_system(
    mut commands: Commands,
    dispelled_effects: Query<Entity, Added<Dispelled>>,
) {
    for effect in dispelled_effects.iter() {
        commands.entity(effect).despawn();
    }
}

fn dispell_on_death_system(
    mut commands: Commands,
    died_characters: Query<Entity, Added<Death>>,
    effects: Query<(Entity, &EffectTarget)>,
) {
    for character in died_characters.iter() {
        for (effect, target) in effects.iter() {
            if character == target.0 {
                commands.entity(effect).insert(Dispelled);
            }
        }
    }
}

fn effect_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut EffectTimer)>,
) {
    for (effect, mut timer) in effects.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            commands.entity(effect).despawn();
        }
    }
}

#[derive(Component, From)]
pub(super) struct EffectTarget(Entity);

#[derive(Component, Deref, DerefMut, From)]
pub(super) struct EffectTimer(Timer);

/// Indticates that the effect is queued for removal
#[derive(Component)]
struct Dispelled;

#[cfg(test)]
mod tests {
    use crate::core::health::HealthChangeEvent;

    use super::*;

    #[test]
    fn effects_cleanup_on_death() {
        let mut app = setup_app();
        let character = app.world.spawn().insert(Death).id();
        let effect = app.world.spawn().insert(EffectTarget(character)).id();

        app.update();
        app.update();

        assert!(
            app.world.get_entity(effect).is_none(),
            "Effect should be removed with character death"
        );
    }

    #[test]
    fn effect_expires() {
        let mut app = setup_app();
        let effect = app
            .world
            .spawn()
            .insert(EffectTimer(Timer::from_seconds(1.0, false)))
            .id();

        app.update();

        let mut timer = app
            .world
            .entity_mut(effect)
            .get_mut::<EffectTimer>()
            .unwrap();
        let duration = timer.duration();
        timer.tick(duration);

        app.update();

        assert!(
            app.world.get_entity(effect).is_none(),
            "Effect should be removed when its duration expires"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_event::<HealthChangeEvent>()
            .add_plugins(MinimalPlugins)
            .add_plugin(EffectPlugin);
        app
    }
}

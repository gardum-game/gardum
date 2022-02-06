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
use std::marker::PhantomData;

use super::AppState;

pub(super) struct EffectTimerPlugin<T: Component, const DURATION: usize>(PhantomData<T>);

impl<T: Component, const DURATION: usize> Plugin for EffectTimerPlugin<T, DURATION> {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(assign_timer_system::<T, DURATION>)
                .with_system(effect_timer_system::<T>),
        );
    }
}

impl<T: Component, const DURATION: usize> Default for EffectTimerPlugin<T, DURATION> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

fn assign_timer_system<T: Component, const DURATION: usize>(
    mut commands: Commands,
    added_effects: Query<Entity, Added<T>>,
) {
    for character in added_effects.iter() {
        commands
            .entity(character)
            .insert(EffectTimer::<T>::new(DURATION));
    }
}

fn effect_timer_system<T: Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut characters: Query<(Entity, &mut EffectTimer<T>)>,
) {
    for (character, mut effect_timer) in characters.iter_mut() {
        effect_timer.tick(time.delta());
        if effect_timer.finished() {
            commands
                .entity(character)
                .remove::<EffectTimer<T>>()
                .remove::<T>();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct EffectTimer<T: Component>(
    #[deref]
    #[deref_mut]
    Timer,
    PhantomData<T>,
);

impl<T: Component> EffectTimer<T> {
    fn new(duration: usize) -> Self {
        Self(
            Timer::from_seconds(duration as f32, false),
            PhantomData::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_timer_assigns_and_removes() {
        let mut app = setup_app();
        let character = app.world.spawn().insert(DummyEffect).id();

        app.update();

        let mut timer = app
            .world
            .entity_mut(character)
            .get_mut::<EffectTimer<DummyEffect>>()
            .expect("Effect timer should be added with effect");
        let duration = timer.duration();
        timer.tick(duration);

        app.update();

        let character_entity = app.world.entity(character);
        assert!(
            !character_entity.contains::<EffectTimer<DummyEffect>>(),
            "Effect timer should be removed when expires"
        );
        assert!(
            !character_entity.contains::<DummyEffect>(),
            "Effect should be removed when the timer expires"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(EffectTimerPlugin::<DummyEffect, 1>::default());

        app
    }

    #[derive(Component)]
    struct DummyEffect;
}

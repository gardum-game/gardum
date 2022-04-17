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

use super::EffectTarget;
use crate::core::{game_state::GameState, health::HealthChangeEvent, Owner};

pub(super) struct PeriodicEffectPlugin;

impl Plugin for PeriodicEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(update_health_system)
                .with_system(periodic_timer_system),
        );
    }
}

fn update_health_system(
    mut effects: Query<(
        &Owner,
        &EffectTarget,
        &PeriodicHealthChange,
        &PeriodicEffectTimer,
    )>,
    mut health_events: EventWriter<HealthChangeEvent>,
) {
    for (instigator, target, delta, timer) in effects.iter_mut() {
        if timer.just_finished() {
            health_events.send(HealthChangeEvent {
                instigator: instigator.0,
                target: target.0,
                delta: delta.0,
            })
        }
    }
}

fn periodic_timer_system(time: Res<Time>, mut effects: Query<&mut PeriodicEffectTimer>) {
    for mut timer in effects.iter_mut() {
        timer.tick(time.delta());
    }
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct PeriodicEffectTimer(Timer);

impl Default for PeriodicEffectTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, true))
    }
}

#[derive(Component, From)]
pub(crate) struct PeriodicHealthChange(i32);

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use std::time::Duration;

    use super::*;

    #[test]
    fn timer_ticks() {
        let mut app = setup_app();
        let effect = app
            .world
            .spawn()
            .insert(PeriodicEffectTimer::default())
            .id();

        app.update();
        app.update();

        let timer = app
            .world
            .entity(effect)
            .get::<PeriodicEffectTimer>()
            .unwrap();
        assert!(
            timer.elapsed() > Duration::default(),
            "Periodic health effect timer should tick"
        );
        assert!(timer.repeating(), "Periodic timer shouldn't stop");
    }

    #[test]
    fn periodic_health_change() {
        let mut app = setup_app();
        let target = app.world.spawn().id();
        let instigator = app.world.spawn().id();

        let mut heal_bundle = DummyPeriodicHealBundle::new(instigator.into(), target.into());
        let time = heal_bundle.periodic_timer.duration() - Duration::from_nanos(1);
        heal_bundle.periodic_timer.tick(time); // Advance timer to almost full duration
        app.world.spawn().insert_bundle(heal_bundle);

        app.update();
        app.update();
        app.update();

        let health_events = app
            .world
            .get_resource_mut::<Events<HealthChangeEvent>>()
            .unwrap();
        let mut event_reader = health_events.get_reader();
        let event = event_reader
            .iter(&health_events)
            .next()
            .expect("Health change event should be triggered");
        assert_eq!(
            event.instigator, instigator,
            "Event instigator should be equal to effect owner"
        );
        assert_eq!(
            event.delta,
            DummyPeriodicHealBundle::DELTA,
            "Event delta should be equal to the effect delta"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(GameState::InGame)
            .add_event::<HealthChangeEvent>()
            .add_plugins(MinimalPlugins)
            .add_plugin(PeriodicEffectPlugin);
        app
    }

    #[derive(Bundle)]
    struct DummyPeriodicHealBundle {
        health_change: PeriodicHealthChange,
        periodic_timer: PeriodicEffectTimer,
        owner: Owner,
        target: EffectTarget,
    }

    impl DummyPeriodicHealBundle {
        const DELTA: i32 = 10;

        fn new(owner: Owner, target: EffectTarget) -> Self {
            Self {
                health_change: DummyPeriodicHealBundle::DELTA.into(),
                periodic_timer: PeriodicEffectTimer::default(),
                owner,
                target,
            }
        }
    }
}

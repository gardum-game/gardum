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
use std::time::Duration;

use super::game_state::GameState;

pub(super) struct DespawnTimerPlugin;

impl Plugin for DespawnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame).with_system(Self::despawn_timer_system),
        );
    }
}

impl DespawnTimerPlugin {
    fn despawn_timer_system(
        mut commands: Commands,
        time: Res<Time>,
        mut timers: Query<(Entity, &mut DespawnTimer)>,
    ) {
        for (entity, mut despawn_timer) in timers.iter_mut() {
            despawn_timer.tick(time.delta());
            if despawn_timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub(super) struct DespawnTimer(Timer);

impl DespawnTimer {
    pub(super) fn from_secs(secs: u64) -> Self {
        let duration = Duration::from_secs(secs);
        Self(Timer::new(duration, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn despawn_timer_from_secs() {
        const SECONDS: u64 = 4;

        let cooldown = DespawnTimer::from_secs(SECONDS);
        assert_eq!(cooldown.duration(), Duration::from_secs(SECONDS));
        assert!(
            !cooldown.finished(),
            "Despawn timer should tick after creation"
        );
    }

    #[test]
    fn despawn_timer_ticks() {
        let mut app = App::new();
        app.add_plugin(TestDespawnTimerPlugin);

        let dummy = app.world.spawn().insert(DespawnTimer::from_secs(1)).id();

        app.update();
        app.update();

        let despawn_timer = app.world.get::<DespawnTimer>(dummy).unwrap();
        assert!(
            despawn_timer.elapsed() > Duration::default(),
            "Despawn timer should tick"
        );
    }

    #[test]
    fn despawn_timer_destroys() {
        let mut app = App::new();
        app.add_plugin(TestDespawnTimerPlugin);

        app.world.spawn().insert(DespawnTimer::default());

        app.update();

        assert_eq!(
            app.world.entities().len(),
            0,
            "Despawn timer should destroy its entity after the time expires"
        );
    }

    struct TestDespawnTimerPlugin;

    impl Plugin for TestDespawnTimerPlugin {
        fn build(&self, app: &mut App) {
            app.add_state(GameState::InGame)
                .add_plugins(MinimalPlugins)
                .add_plugin(DespawnTimerPlugin);
        }
    }
}

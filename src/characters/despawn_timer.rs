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
use std::time::Duration;

use crate::core::AppState;

pub(super) struct DespawnTimerPlugin;

impl Plugin for DespawnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(despawn_timer_system),
        );
    }
}

fn despawn_timer_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.tick(time.delta());
        if despawn_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
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
        let mut app = setup_app();
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
        let mut app = setup_app();
        app.world.spawn().insert(DespawnTimer::default()).id();

        app.update();

        assert_eq!(
            app.world.entities().len(),
            0,
            "Despawn timer should destroy its entity after the time expires"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(DespawnTimerPlugin);
        app
    }
}

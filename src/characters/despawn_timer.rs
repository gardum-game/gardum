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

pub struct DespawnTimerPlugin;

impl Plugin for DespawnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_in_stack_update(AppState::InGame).with_system(despawn_timer_system),
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
pub struct DespawnTimer(Timer);

impl DespawnTimer {
    pub fn from_secs(secs: u64) -> Self {
        let duration = Duration::from_secs(secs);
        Self(Timer::new(duration, false))
    }
}

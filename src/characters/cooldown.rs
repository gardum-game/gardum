/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_in_stack_update(AppState::InGame).with_system(cooldown_system.system()),
        );
    }
}

fn cooldown_system(time: Res<Time>, mut query: Query<&mut Cooldown>) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick(time.delta());
    }
}

#[derive(Deref, DerefMut)]
pub struct Cooldown(Timer);

impl Cooldown {
    pub fn from_secs(secs: u64) -> Self {
        // Setup timer in finished state
        let duration = Duration::from_secs(secs);
        let mut timer = Timer::new(duration, false);
        timer.tick(duration);

        Self(timer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

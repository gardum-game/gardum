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
use std::time::Duration;

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

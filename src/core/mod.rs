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

mod cli;
mod setup;

use bevy::prelude::*;
use heron::PhysicsLayer;

use cli::CliPlugin;
use setup::SetupPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::MainMenu)
            .add_plugin(CliPlugin)
            .add_plugin(SetupPlugin);
    }
}

#[derive(Default)]
pub struct Authority;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    kills: Kills,
    deaths: Deaths,
    damage: Damage,
    healing: Healing,
}

/// Used to keep statistics of the number of kills
#[derive(Default, Debug, PartialEq)]
pub struct Kills(pub usize);

/// Used to keep statistics of the number of deaths
#[derive(Default, Debug, PartialEq)]
pub struct Deaths(pub usize);

/// Used to keep statistics of the damage done
#[derive(Default, Debug, PartialEq)]
pub struct Damage(pub usize);

/// Used to keep statistics of the healing done
#[derive(Default, Debug, PartialEq)]
pub struct Healing(pub usize);

/// Used to store reference to the player
pub struct Player(pub Entity);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Character,
    Projectile,
}

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

use super::{cli::Opts, AppState, Authority};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_server_player_from_opts.system())
            .add_system_set(
                SystemSet::on_enter(AppState::LobbyMenu).with_system(create_server_player.system()),
            );
    }
}

fn create_server_player_from_opts(commands: Commands, opts: Res<Opts>) {
    if opts.subcommand.is_some() {
        create_server_player(commands);
    }
}

fn create_server_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle::default())
        .insert(Authority);
}

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
pub struct PlayerOwner(pub Entity);

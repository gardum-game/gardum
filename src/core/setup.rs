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

use super::{player::Player, Authority};
use crate::{
    characters::heroes::HeroKind,
    core::{cli::Opts, AppState},
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_session_system)
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(select_hero));
    }
}

fn start_session_system(opts: Res<Opts>, mut app_state: ResMut<State<AppState>>) {
    if opts.subcommand.is_some() {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn select_hero(
    player_query: Query<Entity, (With<Authority>, With<Player>)>,
    mut commands: Commands,
) {
    let player = player_query.single();
    commands.entity(player).insert(HeroKind::North);
}

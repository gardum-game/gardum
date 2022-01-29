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
use derive_more::Deref;

use super::{cli::Opts, AppState, Local};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_server_player_from_opts)
            .add_system_set(SystemSet::on_enter(AppState::Lobby).with_system(create_server_player));
    }
}

fn create_server_player_from_opts(commands: Commands, opts: Res<Opts>) {
    if opts.subcommand.is_some() {
        create_server_player(commands);
    }
}

fn create_server_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle {
            nickname: Nickname("New player".to_string()),
            ..Default::default()
        })
        .insert(Local);
}

#[derive(Default, Bundle)]
pub(crate) struct PlayerBundle {
    player: Player,
    nickname: Nickname,
    kills: Kills,
    deaths: Deaths,
    damage: Damage,
    healing: Healing,
}

/// Indicates that the entity is a player
#[derive(Component, Default)]
pub(crate) struct Player;

/// Stores player name
#[derive(Component, Default)]
pub(crate) struct Nickname(pub(crate) String);

/// Used to keep statistics of the number of kills
#[derive(Component, Default, Debug, PartialEq, Deref)]
pub(crate) struct Kills(pub(crate) u32);

/// Used to keep statistics of the number of deaths
#[derive(Component, Default, Debug, PartialEq, Deref)]
pub(crate) struct Deaths(pub(crate) u32);

/// Used to keep statistics of the damage done
#[derive(Component, Default, Debug, PartialEq, Deref)]
pub(crate) struct Damage(pub(crate) u32);

/// Used to keep statistics of the healing done
#[derive(Component, Default, Debug, PartialEq, Deref)]
pub(crate) struct Healing(pub(crate) u32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cli::SubCommand;

    #[test]
    fn server_player_spawns_in_lobby() {
        let mut app = setup_app_in_lobby();
        app.update();

        let mut locals = app.world.query_filtered::<(), With<Local>>();
        locals
            .iter(&app.world)
            .next()
            .expect("Player should be created"); // TODO 0.7: Use single
    }

    #[test]
    fn server_player_spawns_with_host_command() {
        let mut app = setup_app_with_host_command();
        app.update();

        let mut locals = app.world.query_filtered::<(), With<Local>>();
        locals
            .iter(&app.world)
            .next()
            .expect("Player should be created"); // TODO 0.7: Use single
    }

    fn setup_app_in_lobby() -> App {
        let mut app = App::new();
        app.init_resource::<Opts>()
            .add_state(AppState::Lobby)
            .add_plugin(PlayerPlugin);
        app
    }

    fn setup_app_with_host_command() -> App {
        let mut app = App::new();
        app.insert_resource(Opts {
            subcommand: Some(SubCommand::Host),
        })
        .add_state(AppState::Menu)
        .add_plugin(PlayerPlugin);
        app
    }
}

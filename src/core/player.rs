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

fn create_server_player_from_opts(mut commands: Commands, opts: Res<Opts>) {
    if opts.subcommand.is_some() {
        let mut player = commands.spawn_bundle(PlayerBundle::default());
        player.insert(Local);
        if let Some(hero_kind) = opts.preselect_hero {
            player.insert(hero_kind);
        }
    }
}

fn create_server_player(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle::default()).insert(Local);
}

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    name: Name,
    player: Player,
    kills: Kills,
    deaths: Deaths,
    damage: Damage,
    healing: Healing,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: "New player".into(),
            player: Player::default(),
            kills: Kills::default(),
            deaths: Deaths::default(),
            damage: Damage::default(),
            healing: Healing::default(),
        }
    }
}

/// Indicates that the entity is a player
#[derive(Component, Default)]
pub(crate) struct Player;

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
    use strum::IntoEnumIterator;

    use super::*;
    use crate::core::{character::hero::HeroKind, cli::SubCommand};

    #[test]
    fn server_player_spawns_in_lobby() {
        let mut app = setup_app();
        app.add_state(AppState::Lobby).init_resource::<Opts>();

        app.update();

        let mut locals = app
            .world
            .query_filtered::<(), (With<Local>, With<Player>, Without<HeroKind>)>();
        locals
            .iter(&app.world)
            .next()
            .expect("Local player should be created without preselected hero"); // TODO 0.7: Use single
    }

    #[test]
    fn server_player_spawns_with_host_command() {
        let mut app = setup_app();
        app.insert_resource(Opts {
            subcommand: Some(SubCommand::Host),
            preselect_hero: Some(HeroKind::iter().next().unwrap()),
        })
        .add_state(AppState::Menu);

        app.update();

        let mut locals = app
            .world
            .query_filtered::<&HeroKind, (With<Local>, With<Player>)>();
        let hero_kind = locals
            .iter(&app.world)
            .next()
            .expect("Local player should be created with preselected hero"); // TODO 0.7: Use single
        assert_eq!(
            *hero_kind,
            HeroKind::iter().next().unwrap(),
            "Player should have the specified preselected hero"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(PlayerPlugin);
        app
    }
}

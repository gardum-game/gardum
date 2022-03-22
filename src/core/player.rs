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
#[cfg(feature = "gi")]
use bevy_hikari::NotGiCaster;
use derive_more::Deref;

use super::{
    character::hero::HeroKind, cli::Opts, game_state::GameState, server_settings::ServerSettings,
    Authority,
};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_server_player_from_opts)
            .add_system_set(
                SystemSet::on_enter(GameState::Lobby).with_system(create_server_player),
            );

        #[cfg(feature = "gi")]
        app.add_system(player_not_cast_gi);
    }
}

fn create_server_player_from_opts(
    mut commands: Commands,
    opts: Res<Opts>,
    server_settings: Res<ServerSettings>,
) {
    if opts.subcommand.is_some() {
        let mut player = commands.spawn_bundle(PlayerBundle::default());
        player.insert(Authority);
        if server_settings.random_heroes {
            player.insert(HeroKind::North); // TODO: Implement random selection when there are more than one hero
        }
    }
}

fn create_server_player(mut commands: Commands) {
    commands
        .spawn_bundle(PlayerBundle::default())
        .insert(Authority);
}

#[cfg(feature = "gi")]
fn player_not_cast_gi(
    mut commands: Commands,
    players: Query<(Entity, &Player), Without<NotGiCaster>>,
) {
    for (entity, _) in players.iter() {
        commands.entity(entity).insert(NotGiCaster);
    }
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
    use super::*;
    use crate::core::{character::hero::HeroKind, cli::SubCommand};

    #[test]
    fn player_spawns_in_lobby() {
        let mut app = setup_app();
        app.add_state(GameState::Lobby)
            .init_resource::<Opts>()
            .init_resource::<ServerSettings>();

        app.update();

        let mut local_player = app
            .world
            .query_filtered::<(), (With<Authority>, With<Player>, Without<HeroKind>)>();
        local_player
            .iter(&app.world)
            .next()
            .expect("Local player should be created without hero"); // TODO 0.7: Use single
    }

    #[test]
    fn player_spawns_from_cli() {
        let mut app = setup_app();
        app.insert_resource(ServerSettings {
            random_heroes: true,
            ..ServerSettings::default()
        })
        .insert_resource(Opts {
            subcommand: Some(SubCommand::Connect),
        })
        .add_state(GameState::Menu);

        app.update();

        let mut local_player = app
            .world
            .query_filtered::<(), (With<Authority>, With<Player>, With<HeroKind>)>();
        local_player
            .iter(&app.world)
            .next()
            .expect("Local player should be created with hero"); // TODO 0.7: Use single
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(PlayerPlugin);
        app
    }
}

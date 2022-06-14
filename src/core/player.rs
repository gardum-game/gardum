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
use iyes_loopless::prelude::*;

use super::{network::NetworkingState, Authority};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(NetworkingState::Connected, Self::spawn_player_system)
            .add_enter_system(NetworkingState::Hosting, Self::spawn_player_system)
            .add_exit_system(NetworkingState::Connected, Self::despawn_players_system)
            .add_exit_system(NetworkingState::Hosting, Self::despawn_players_system);
    }
}

impl PlayerPlugin {
    fn spawn_player_system(mut commands: Commands) {
        commands
            .spawn_bundle(PlayerBundle::default())
            .insert(Authority);
    }

    fn despawn_players_system(mut commands: Commands, players: Query<Entity, With<Player>>) {
        for player in players.iter() {
            commands.entity(player).despawn_recursive();
        }
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

    #[test]
    fn player_spawns_despawns() {
        let mut app = App::new();
        app.add_plugin(TestPlayerPlugin);

        for state in [NetworkingState::Connected, NetworkingState::Hosting] {
            app.world.insert_resource(NextState(state));

            app.update();

            let mut local_player = app
                .world
                .query_filtered::<Entity, (With<Authority>, With<Player>)>();
            local_player.iter(&app.world).next().unwrap_or_else(|| {
                panic!(
                    "Local player should be created after entering {:?} state",
                    state
                )
            }); // TODO 0.8: Use single

            app.world
                .insert_resource(NextState(NetworkingState::NoSocket));

            app.update();

            assert!(
                local_player.iter(&app.world).next().is_none(), // TODO 0.8: Use single
                "Local player should be removed after entering {:?} state",
                NetworkingState::NoSocket
            );
        }
    }

    struct TestPlayerPlugin;

    impl Plugin for TestPlayerPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(NetworkingState::NoSocket)
                .add_plugin(PlayerPlugin);
        }
    }
}

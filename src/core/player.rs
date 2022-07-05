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
use bevy_renet::renet::{RenetClient, RenetServer};
use iyes_loopless::prelude::*;

use super::Authority;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::spawn_player_system.run_if_resource_added::<RenetServer>())
            .add_system(Self::despawn_players_system.run_if_resource_removed::<RenetServer>())
            .add_system(Self::despawn_players_system.run_if_resource_removed::<RenetClient>());
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
    use crate::core::network::tests::{NetworkPreset, TestNetworkPlugin};

    use super::*;

    #[test]
    fn player_spawns_despawns_on_server() {
        let mut app = App::new();
        app.add_plugin(PlayerPlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Server));

        app.update();

        let local_player = app
            .world
            .query_filtered::<Entity, (With<Authority>, With<Player>)>()
            .iter(&app.world)
            .next()
            .expect("Local player should be created after inserting server resource"); // TODO 0.8: Use single

        app.world.remove_resource::<RenetServer>();

        app.update();

        assert!(
            app.world.get_entity(local_player).is_none(),
            "Local player should be removed after removing server resource",
        );
    }

    #[test]
    fn player_despawns_on_client() {
        let mut app = App::new();
        app.add_plugin(PlayerPlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Client));

        // On client spawned player is replicated, spawn it manually to test removal
        let local_player = app.world.spawn().insert(Player).insert(Authority).id();

        app.update();

        app.world.remove_resource::<RenetClient>();

        app.update();

        assert!(
            app.world.get_entity(local_player).is_none(),
            "Local player should be removed after removing client resource",
        );
    }
}

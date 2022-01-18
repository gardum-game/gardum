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
use strum::EnumIter;

use crate::{
    characters::heroes::{HeroBundle, HeroKind, OwnerPlayer},
    core::{player::Player, AppState},
};

pub(super) struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMode::Deathmatch)
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(spawn_system));
    }
}

fn spawn_system(
    spawn_point_query: Query<&SpawnPoint>,
    player_query: Query<(Entity, &HeroKind), (Added<HeroKind>, With<Player>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (player, hero_kind) in player_query.iter() {
        for spawn_point in spawn_point_query.iter() {
            let hero = HeroBundle::hero(
                *hero_kind,
                OwnerPlayer(player),
                Transform::from_translation(spawn_point.0),
                &mut commands,
                &mut meshes,
                &mut materials,
            );
            commands.spawn_bundle(hero);
        }
    }
}

#[derive(Component)]
pub(crate) struct SpawnPoint(pub(crate) Vec3);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub(crate) enum GameMode {
    Disabled,
    Deathmatch,
}

impl GameMode {
    pub(crate) const fn slots_count(self) -> u8 {
        match self {
            GameMode::Disabled | GameMode::Deathmatch => 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;
    use crate::{core::AppState, test_utils::HeadlessRenderPlugin};

    #[test]
    fn hero_spawns() {
        const SPAWN_POINT: Vec3 = Vec3::ONE;
        let kind = HeroKind::iter().next().unwrap();

        let mut app = setup_app();
        let player = app.world.spawn().insert(Player).insert(kind).id();
        app.world.spawn().insert(SpawnPoint(SPAWN_POINT)).id();

        app.update();

        let mut query = app
            .world
            .query_filtered::<(&Transform, &OwnerPlayer, &HeroKind), Without<Player>>();

        let (transform, owner_player, hero_kind) = query
            .iter(&app.world)
            .next()
            .expect("Hero should be spawned"); // TODO 0.7: Use single
        assert_eq!(
            transform.translation, SPAWN_POINT,
            "Hero should be spawned at the specified location"
        );
        assert_eq!(
            owner_player.0, player,
            "Hero should have the specified player"
        );
        assert_eq!(*hero_kind, kind, "Hero should have the player's kind");
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(GameModesPlugin);

        app
    }
}

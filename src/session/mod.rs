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
    characters::{heroes::HeroKind, CharacterBundle},
    core::AppState,
};

pub(super) struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMode::Deathmatch)
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(spawn_system));
    }
}

fn spawn_system(
    spawn_points: Query<&SpawnPoint>,
    players: Query<(Entity, &HeroKind), Added<HeroKind>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (player, hero_kind) in players.iter() {
        // TODO: determine best spawn position based on other characters location
        let spawn_point = spawn_points
            .iter()
            .next()
            .expect("Unable to find any spawn points");

        let hero = CharacterBundle::hero(
            *hero_kind,
            Transform::from_translation(spawn_point.0),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        commands.entity(player).insert_bundle(hero);
    }
}

#[derive(Component)]
pub(crate) struct SpawnPoint(pub(crate) Vec3);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub(crate) enum GameMode {
    Deathmatch,
}

impl GameMode {
    pub(crate) const fn slots_count(self) -> u8 {
        match self {
            GameMode::Deathmatch => 10,
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
        let hero_kind = HeroKind::iter().next().unwrap();

        let mut app = setup_app();
        let player = app.world.spawn().insert(hero_kind).id();
        app.world.spawn().insert(SpawnPoint(SPAWN_POINT)).id();

        app.update();

        let transform = app
            .world
            .get::<Transform>(player)
            .expect("Hero should be spawned");
        assert_eq!(
            transform.translation, SPAWN_POINT,
            "Hero should be spawned at the specified location"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(SessionPlugin);

        app
    }
}

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
use derive_more::{Deref, DerefMut};

use crate::{
    characters::{heroes::HeroKind, CharacterBundle},
    core::{player::Deaths, AppState},
};

pub(super) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_system)
                .with_system(assign_respawn_system)
                .with_system(respawn_system),
        );
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

fn assign_respawn_system(
    mut died_players: Query<(Entity, ChangeTrackers<Deaths>)>,
    mut commands: Commands,
) {
    for (player, deaths_trackers) in died_players.iter_mut() {
        if deaths_trackers.is_changed() && !deaths_trackers.is_added() {
            commands.entity(player).insert(RespawnTimer::default());
        }
    }
}

fn respawn_system(
    time: Res<Time>,
    spawn_points: Query<&SpawnPoint>,
    mut dead_players: Query<(Entity, &mut Transform, &mut RespawnTimer)>,
    mut commands: Commands,
) {
    for (player, mut transform, mut respawn_timer) in dead_players.iter_mut() {
        respawn_timer.tick(time.delta());
        if respawn_timer.finished() {
            commands.entity(player).remove::<RespawnTimer>();
            // TODO: determine best spawn position based on other characters location
            let spawn_point = spawn_points
                .iter()
                .next()
                .expect("Unable to find any spawn points");

            transform.translation = spawn_point.0;
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct RespawnTimer(Timer);

impl Default for RespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, false))
    }
}

#[derive(Component)]
pub(crate) struct SpawnPoint(pub(crate) Vec3);

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

    #[test]
    fn respawn_asigns() {
        let mut app = setup_app();
        let player = app.world.spawn().insert(Deaths::default()).id();

        app.update();

        assert!(
            !app.world.entity(player).contains::<RespawnTimer>(),
            "Player shouldn't have respawn timer assigned until first death"
        );

        app.world.entity_mut(player).get_mut::<Deaths>().unwrap().0 += 1;

        app.update();

        assert!(
            app.world.entity(player).contains::<RespawnTimer>(),
            "Player should have respawn timer assigned after death"
        );
    }

    #[test]
    fn player_respawns() {
        let mut app = setup_app();
        let player = app
            .world
            .spawn()
            .insert(RespawnTimer::default())
            .insert(Transform::default())
            .id();
        let spawn_point = app.world.spawn().insert(SpawnPoint(Vec3::ONE)).id();

        app.update();
        app.update();

        assert!(
            app.world
                .entity(player)
                .get::<RespawnTimer>()
                .unwrap()
                .elapsed_secs()
                > 0.0,
            "Respawn timer should tick"
        );

        app.world
            .entity_mut(player)
            .get_mut::<RespawnTimer>()
            .unwrap()
            .tick(RespawnTimer::default().duration());
        app.update();

        assert!(
            !app.world.entity(player).contains::<RespawnTimer>(),
            "Respawn timer should be removed"
        );

        let player_translation = app
            .world
            .entity(player)
            .get::<Transform>()
            .unwrap()
            .translation;
        assert_eq!(
            player_translation,
            app.world.entity(spawn_point).get::<SpawnPoint>().unwrap().0,
            "Player should be moved to spawn point"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(SpawnPlugin);

        app
    }
}

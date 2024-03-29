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
use derive_more::From;
use iyes_loopless::prelude::*;

use crate::core::{
    game_state::{GameState, InGameOnly},
    health::Death,
    hero::{HeroBundle, HeroKind},
    network::server,
    player::Player,
};

pub(super) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(
            GameState::InGame,
            Self::randomize_heroes_system.run_if(server::random_heroes),
        )
        .add_system(Self::spawn_system.run_in_state(GameState::InGame))
        .add_system(Self::assign_respawn_timer_system.run_in_state(GameState::InGame))
        .add_system(Self::respawn_system.run_in_state(GameState::InGame));
    }
}

impl SpawnPlugin {
    fn randomize_heroes_system(mut commands: Commands, players: Query<Entity, Added<Player>>) {
        for player in players.iter() {
            commands.entity(player).insert(HeroKind::North); // TODO: Implement random selection when there are more than one hero
        }
    }

    fn spawn_system(
        mut commands: Commands,
        spawn_points: Query<&SpawnPoint>,
        players: Query<(Entity, &HeroKind), Added<HeroKind>>,
    ) {
        for (player, &hero_kind) in players.iter() {
            // TODO: determine best spawn position based on other characters location
            let spawn_point = spawn_points
                .iter()
                .next()
                .expect("Unable to find any spawn points");

            commands
                .entity(player)
                .insert_bundle(HeroBundle::new(hero_kind, spawn_point.0));
        }
    }

    fn assign_respawn_timer_system(
        mut commands: Commands,
        mut died_players: Query<Entity, Added<Death>>,
    ) {
        for player in died_players.iter_mut() {
            commands.entity(player).insert(RespawnTimer::default());
        }
    }

    fn respawn_system(
        mut commands: Commands,
        time: Res<Time>,
        spawn_points: Query<&SpawnPoint>,
        mut dead_players: Query<(Entity, &mut Transform, &mut RespawnTimer)>,
    ) {
        for (player, mut transform, mut respawn_timer) in dead_players.iter_mut() {
            respawn_timer.tick(time.delta());
            if respawn_timer.just_finished() {
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
}

#[derive(Component, Deref, DerefMut)]
struct RespawnTimer(Timer);

impl Default for RespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, false))
    }
}

#[derive(Component, From)]
struct SpawnPoint(pub(crate) Vec3);

#[derive(Bundle)]
pub(crate) struct SpawnPointBundle {
    spawn_point: SpawnPoint,
    ingame_only: InGameOnly,
}

impl SpawnPointBundle {
    pub(crate) fn new(translation: Vec3) -> Self {
        Self {
            spawn_point: translation.into(),
            ingame_only: InGameOnly,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use strum::IntoEnumIterator;

    use super::*;
    use crate::core::{
        game_state::GameState, headless::HeadlessRenderPlugin, network::server::ServerSettings,
    };

    #[test]
    fn heroes_randomization() {
        let mut app = App::new();
        app.add_plugin(TestSpawnPlugin);

        app.insert_resource(ServerSettings {
            random_heroes: true,
            ..ServerSettings::default()
        });
        app.world.spawn().insert(SpawnPoint(Vec3::ZERO));

        let player = app.world.spawn().insert(Player).id();

        app.update();

        assert!(
            app.world.entity(player).get::<HeroKind>().is_some(),
            "Hero must be randomized if heroes randomization is enabled on the server"
        );
    }

    #[test]
    fn hero_spawns() {
        let mut app = App::new();
        app.add_plugin(TestSpawnPlugin);

        const SPAWN_POINT: Vec3 = Vec3::ONE;
        app.world.spawn().insert(SpawnPoint(SPAWN_POINT));

        for hero_kind in HeroKind::iter() {
            let player = app.world.spawn().insert(hero_kind).id();

            app.update();

            let transform = app
                .world
                .get::<Transform>(player)
                .expect("Hero should be inserted to the player");
            assert_eq!(
                transform.translation, SPAWN_POINT,
                "Hero should be spawned at the specified location"
            );
        }
    }

    #[test]
    fn respawn_asigns() {
        let mut app = App::new();
        app.add_plugin(TestSpawnPlugin);

        let player = app.world.spawn().id();

        app.update();

        assert!(
            !app.world.entity(player).contains::<RespawnTimer>(),
            "Player shouldn't have respawn timer assigned until first death"
        );

        app.world.entity_mut(player).insert(Death);

        app.update();

        assert!(
            app.world.entity(player).contains::<RespawnTimer>(),
            "Player should have respawn timer assigned after death"
        );
    }

    #[test]
    fn player_respawns() {
        let mut app = App::new();
        app.add_plugin(TestSpawnPlugin);

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

        let mut respawn_timer = app.world.get_mut::<RespawnTimer>(player).unwrap();
        let duration_left = respawn_timer.duration() - respawn_timer.elapsed();
        respawn_timer.tick(duration_left - Duration::from_nanos(1)); // Tick to almost end to trigger just_finished inside the system
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

    struct TestSpawnPlugin;

    impl Plugin for TestSpawnPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<ServerSettings>()
                .add_loopless_state(GameState::InGame)
                .add_plugin(HeadlessRenderPlugin)
                .add_plugin(SpawnPlugin);
        }
    }
}

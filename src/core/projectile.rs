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
use heron::{CollisionLayers, CollisionShape, Collisions, RigidBody, Velocity};

use super::{despawn_timer::DespawnTimer, game_state::GameState, CollisionLayer};

pub(super) struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(collision_system));
    }
}

fn collision_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Collisions), Changed<Collisions>>,
) {
    for (projectile, collisions) in projectiles.iter() {
        if !collisions.is_empty() {
            commands.entity(projectile).despawn();
        }
    }
}

#[derive(Bundle)]
pub(super) struct ProjectileBundle {
    pub(super) name: Name,
    pub(super) rigid_body: RigidBody,
    pub(super) shape: CollisionShape,
    pub(super) collision_layers: CollisionLayers,
    pub(super) velocity: Velocity,
    pub(super) projectile: Projectile,
    pub(super) despawn_timer: DespawnTimer,
    pub(super) collisions: Collisions,

    #[bundle]
    pub(super) pbr: PbrBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            name: "Projectile".into(),
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::all_masks::<CollisionLayer>()
                .without_mask(CollisionLayer::Projectile)
                .with_group(CollisionLayer::Projectile),
            velocity: Velocity::default(),
            projectile: Projectile,
            despawn_timer: DespawnTimer::from_secs(4),
            collisions: Collisions::default(),
            pbr: PbrBundle::default(),
        }
    }
}

#[derive(Component)]
pub(super) struct Projectile;

#[cfg(test)]
mod tests {
    use heron::PhysicsPlugin;

    use super::*;

    #[test]
    fn projectile_moves() {
        let mut app = setup_app();
        let projectile_entity = app
            .world
            .spawn()
            .insert_bundle(ProjectileBundle {
                velocity: Velocity::from_linear(Vec3::ONE),
                ..Default::default()
            })
            .id();

        app.update();
        app.update();

        let transform = app.world.get::<Transform>(projectile_entity).unwrap();
        assert!(
            transform.translation.length() > 0.0,
            "Projectile should be moved by velocity"
        );
    }

    #[test]
    fn projectile_collides() {
        let mut app = setup_app();
        let projectile = app
            .world
            .spawn()
            .insert_bundle(ProjectileBundle::default())
            .id();
        let object = app.world.spawn().insert_bundle(DummyBundle::default()).id();

        app.update();
        app.update();
        app.update();

        assert!(
            app.world.get_entity(projectile).is_none(),
            "Projectile should be destroyed when colliding with other objects"
        );
        assert!(
            app.world.get_entity(object).is_some(),
            "Colliding object should exist"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(GameState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(ProjectilePlugin);
        app
    }

    #[derive(Bundle)]
    struct DummyBundle {
        rigid_body: RigidBody,
        shape: CollisionShape,
        transform: Transform,
        global_transform: GlobalTransform,
    }

    impl Default for DummyBundle {
        fn default() -> Self {
            Self {
                rigid_body: RigidBody::Static,
                shape: CollisionShape::default(),
                transform: Transform::default(),
                global_transform: GlobalTransform::default(),
            }
        }
    }
}

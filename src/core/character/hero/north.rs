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

use bevy::{ecs::system::EntityCommands, prelude::*, render::camera::Camera};
use heron::{CollisionShape, Collisions, Velocity};

use crate::core::{
    ability::{Activator, IconPath},
    character::{character_direction, CharacterBundle},
    cooldown::Cooldown,
    game_state::GameState,
    health::{Health, HealthChangeEvent},
    projectile::ProjectileBundle,
    settings::ControlAction,
    AssetCommands, Owner,
};

const PROJECTILE_SPEED: f32 = 20.0;
const FROST_BOLT_SPAWN_OFFSET: f32 = 4.0;
const FROST_BOLT_DAMAGE: i32 = -20;
const FROST_PATH_IMPULSE: f32 = 130.0;

pub(super) struct NorthPlugin;

impl Plugin for NorthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(frost_bolt_system)
                .with_system(frost_bolt_hit_system)
                .with_system(frost_path_system),
        );
    }
}

fn frost_bolt_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    abilities: Query<(Entity, &Activator), With<FrostBoltAbility>>,
    characters: Query<&Transform>,
    cameras: Query<&Transform, With<Camera>>,
) {
    for (ability, activator) in abilities.iter() {
        let camera_transform = cameras.single();
        let character_transform = characters.get(activator.0).unwrap();

        commands
            .spawn_bundle(ProjectileBundle::frost_bolt(
                camera_transform,
                character_transform,
                &mut meshes,
                &mut materials,
            ))
            .insert(FrostBoltAbility)
            .insert(Owner(activator.0));
        commands.entity(ability).remove::<Activator>();
    }
}

fn frost_bolt_hit_system(
    mut health_events: EventWriter<HealthChangeEvent>,
    projectiles: Query<(&Owner, &Collisions), (With<FrostBoltAbility>, Changed<Collisions>)>,
    health: Query<(), With<Health>>,
) {
    for (owner, collisions) in projectiles.iter() {
        if let Some(first_collision) = collisions.iter().next() {
            if health.get(first_collision).is_ok() {
                health_events.send(HealthChangeEvent {
                    instigator: owner.0,
                    target: first_collision,
                    delta: FROST_BOLT_DAMAGE,
                });
            }
        }
    }
}

fn frost_path_system(
    mut commands: Commands,
    abilities: Query<(Entity, &Activator), With<FrostPathAbility>>,
    mut characters: Query<&mut Velocity>,
    cameras: Query<&Transform, With<Camera>>,
) {
    for (ability, activator) in abilities.iter() {
        let camera_transform = cameras.single();
        let mut velocity = characters.get_mut(activator.0).unwrap();
        velocity.linear += character_direction(camera_transform.rotation) * FROST_PATH_IMPULSE;

        commands.entity(ability).remove::<Activator>();
    }
}

#[derive(Bundle)]
struct FrostBoltBundle {
    name: Name,
    frost_bolt_ability: FrostBoltAbility,
    icon: IconPath,
    action: ControlAction,
    cooldown: Cooldown,
}

impl Default for FrostBoltBundle {
    fn default() -> Self {
        Self {
            name: "Frost Bolt Ability".into(),
            frost_bolt_ability: FrostBoltAbility,
            icon: "character/hero/north/frost_bolt.png".into(),
            action: ControlAction::BaseAttack,
            cooldown: Cooldown::from_secs(4),
        }
    }
}

#[derive(Component)]
struct FrostBoltAbility;

#[derive(Bundle)]
struct FrostPathBundle {
    name: Name,
    frost_path_ability: FrostPathAbility,
    icon: IconPath,
    action: ControlAction,
    cooldown: Cooldown,
}

impl Default for FrostPathBundle {
    fn default() -> Self {
        Self {
            name: "Frost Path Ability".into(),
            frost_path_ability: FrostPathAbility,
            icon: "character/hero/north/frost_path.png".into(),
            action: ControlAction::Ability1,
            cooldown: Cooldown::from_secs(4),
        }
    }
}

#[derive(Component)]
struct FrostPathAbility;

impl<'w, 's> AssetCommands<'w, 's> {
    pub(crate) fn insert_north<'a>(
        &'a mut self,
        player: Entity,
        transform: Transform,
    ) -> EntityCommands<'w, 's, 'a> {
        let abilities = vec![
            self.commands.spawn_bundle(FrostBoltBundle::default()).id(),
            self.commands.spawn_bundle(FrostPathBundle::default()).id(),
        ];

        let mut entity_commands = self.commands.entity(player);
        entity_commands.insert_bundle(CharacterBundle {
            abilities: abilities.into(),
            pbr: PbrBundle {
                mesh: self.meshes.add(Mesh::from(shape::Capsule::default())),
                material: self.materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform,
                ..Default::default()
            },
            shape: CollisionShape::Capsule {
                half_segment: 0.5,
                radius: 0.5,
            },
            ..Default::default()
        });
        entity_commands
    }
}

impl ProjectileBundle {
    fn frost_bolt(
        camera_transform: &Transform,
        character_transform: &Transform,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            shape: CollisionShape::Capsule {
                half_segment: 0.5,
                radius: 0.5,
            },
            velocity: Velocity::from_linear(
                camera_transform.rotation * -Vec3::Z * PROJECTILE_SPEED,
            ),
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform: Transform {
                    translation: character_transform.translation
                        + camera_transform.rotation * -Vec3::Z * FROST_BOLT_SPAWN_OFFSET,
                    rotation: camera_transform.rotation
                        * Quat::from_rotation_x(90.0_f32.to_radians()),
                    scale: character_transform.scale,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use bevy::app::Events;
    use heron::PhysicsPlugin;

    use super::*;
    use crate::{core::projectile::Projectile, test_utils::HeadlessRenderPlugin};

    #[test]
    fn frost_bolt() {
        let mut app = setup_app();
        let instigator = app
            .world
            .spawn()
            .insert(Transform::from_translation(Vec3::ONE))
            .id();
        let ability = app
            .world
            .spawn()
            .insert_bundle(FrostBoltBundle::default())
            .insert(Activator(instigator))
            .id();
        let camera = app
            .world
            .spawn()
            .insert_bundle(DummyCameraBundle::default())
            .id();

        app.update();

        let mut projectiles = app.world.query_filtered::<&Transform, With<Projectile>>();
        let projectile_transform = *projectiles.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
        let character_transform = app.world.get::<Transform>(instigator).unwrap();

        assert_relative_eq!(
            character_transform.translation.x,
            projectile_transform.translation.x
        );
        assert_relative_eq!(
            character_transform.translation.y + FROST_BOLT_SPAWN_OFFSET,
            projectile_transform.translation.y
        );
        assert_relative_eq!(
            character_transform.translation.z,
            projectile_transform.translation.z
        );
        assert_eq!(
            character_transform.scale, projectile_transform.scale,
            "Spawned projectile must be of the same scale as the character"
        );

        let camera_trasnform = app.world.get::<Transform>(camera).unwrap();
        assert_relative_eq!(
            projectile_transform.rotation,
            camera_trasnform.rotation * Quat::from_rotation_x(90.0_f32.to_radians()),
        );

        assert!(
            !app.world.entity(ability).contains::<Activator>(),
            "Activator component should be removed from the ability",
        );

        let target = app
            .world
            .spawn()
            .insert_bundle(CharacterBundle {
                pbr: PbrBundle {
                    transform: projectile_transform,
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        app.update();
        app.update();
        app.update();

        let health_events = app
            .world
            .get_resource::<Events<HealthChangeEvent>>()
            .unwrap();
        let mut reader = health_events.get_reader();
        let event = reader
            .iter(&health_events)
            .next()
            .expect("Health change event should be emitted");

        assert_eq!(
            event.instigator, instigator,
            "Instigator should be equal to specified"
        );
        assert_eq!(event.target, target, "Target should be equal to specified");
        assert_eq!(
            event.delta, FROST_BOLT_DAMAGE,
            "Damage should be equal to frost bolt damage"
        );
    }

    #[test]
    fn frost_path() {
        let mut app = setup_app();
        let character = app
            .world
            .spawn()
            .insert(Transform::default())
            .insert(Velocity::from_linear(Vec3::ZERO))
            .id();
        let ability = app
            .world
            .spawn()
            .insert_bundle(FrostPathBundle::default())
            .insert(Activator(character))
            .id();
        let camera = app
            .world
            .spawn()
            .insert_bundle(DummyCameraBundle::default())
            .id();

        app.update();

        let velocity = app.world.get::<Velocity>(character).unwrap();
        let camera_transform = app.world.get::<Transform>(camera).unwrap();

        assert_eq!(
            velocity.linear,
            character_direction(camera_transform.rotation) * FROST_PATH_IMPULSE,
            "Character should recieve impulse in camera direction"
        );

        assert!(
            !app.world.entity(ability).contains::<Activator>(),
            "Activator component should be removed from the ability",
        )
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_event::<HealthChangeEvent>()
            .add_state(GameState::InGame)
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(NorthPlugin);

        app
    }

    #[derive(Bundle)]
    struct DummyCameraBundle {
        transform: Transform,
        camera: Camera,
    }

    impl Default for DummyCameraBundle {
        fn default() -> Self {
            Self {
                transform: Transform::from_rotation(Quat::from_rotation_x(90_f32.to_radians())),
                camera: Camera::default(),
            }
        }
    }
}

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

use bevy::{prelude::*, render::camera::Camera};
use heron::{CollisionShape, Velocity};

use crate::core::{
    ability::{Abilities, ActivationEvent},
    character::{CharacterBundle, Owner},
    character_action::CharacterAction,
    cooldown::Cooldown,
    health::DamageEvent,
    projectile::{ProjectileBundle, ProjectileHitEvent},
    AppState, IconPath,
};

const PROJECTILE_SPEED: f32 = 20.0;
const FROST_BOLT_SPAWN_OFFSET: f32 = 4.0;
const FROST_BOLT_DAMAGE: u32 = 20;

pub(super) struct NorthPlugin;

impl Plugin for NorthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(frost_bolt_system)
                .with_system(frost_bolt_hit_system),
        );
    }
}

fn frost_bolt_system(
    mut commands: Commands,
    mut events: EventReader<ActivationEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    frost_bolts: Query<(), With<FrostBoltAbility>>,
    character_transforms: Query<&Transform>,
    camera_transforms: Query<&Transform, With<Camera>>,
) {
    for event in events
        .iter()
        .filter(|event| frost_bolts.get(event.ability).is_ok())
    {
        let camera_transform = camera_transforms.single();
        let character_transform = character_transforms.get(event.character).unwrap();

        commands
            .spawn_bundle(ProjectileBundle::frost_bolt(
                camera_transform,
                character_transform,
                &mut meshes,
                &mut materials,
            ))
            .insert(FrostBoltProjectile)
            .insert(Owner(event.character));
    }
}

fn frost_bolt_hit_system(
    mut hit_events: EventReader<ProjectileHitEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    frost_bolt_owners: Query<&Owner, With<FrostBoltProjectile>>,
) {
    for event in hit_events.iter() {
        if let Ok(character) = frost_bolt_owners.get(event.projectile) {
            damage_events.send(DamageEvent {
                instigator: character.0,
                target: event.target,
                damage: FROST_BOLT_DAMAGE,
            });
        }
    }
}

#[derive(Bundle)]
struct FrostBoltBundle {
    frost_bolt_ability: FrostBoltAbility,
    icon: IconPath,
    action: CharacterAction,
    cooldown: Cooldown,
}

impl Default for FrostBoltBundle {
    fn default() -> Self {
        Self {
            frost_bolt_ability: FrostBoltAbility,
            icon: "charaters/heroes/north/frost_bolt.png".into(),
            action: CharacterAction::BaseAttack,
            cooldown: Cooldown::from_secs(4),
        }
    }
}

#[derive(Component)]
struct FrostBoltAbility;

#[derive(Component)]
struct FrostBoltProjectile;

impl CharacterBundle {
    pub(super) fn north(
        transform: Transform,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            abilities: Abilities(vec![commands.spawn_bundle(FrostBoltBundle::default()).id()]),
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform,
                ..Default::default()
            },
            shape: CollisionShape::Capsule {
                half_segment: 0.5,
                radius: 0.5,
            },
            ..Default::default()
        }
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

    use super::*;
    use crate::{core::projectile::Projectile, test_utils::HeadlessRenderPlugin};

    #[test]
    fn frost_bolt() {
        let mut app = setup_app();
        let ability = app
            .world
            .spawn()
            .insert_bundle(FrostBoltBundle::default())
            .id();
        let character = app
            .world
            .spawn()
            .insert(Transform::from_translation(Vec3::ONE))
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default())
            .id();

        let mut events = app
            .world
            .get_resource_mut::<Events<ActivationEvent>>()
            .unwrap();

        events.send(ActivationEvent { character, ability });

        app.update();
        app.update();

        let mut character_transforms = app.world.query_filtered::<&Transform, Without<Camera>>();
        let mut projectile_transforms = app.world.query_filtered::<&Transform, With<Projectile>>();
        let mut camera_transforms = app.world.query_filtered::<&Transform, With<Camera>>();

        let character_transform = character_transforms.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
        let projectile_transform = projectile_transforms.iter(&app.world).next().unwrap(); // TODO 0.7: Use single

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

        let camera_trasnform = camera_transforms.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
        assert_eq!(
            projectile_transform.rotation,
            camera_trasnform.rotation * Quat::from_rotation_x(90.0_f32.to_radians()),
            "Spawned projectile must be turned towards the camera."
        );
    }

    #[test]
    fn frost_bolt_hit() {
        let mut app = setup_app();
        let instigator = app.world.spawn().id();
        let projectile = app
            .world
            .spawn()
            .insert(FrostBoltProjectile)
            .insert(Owner(instigator))
            .id();
        let target = app.world.spawn().id();

        let mut events = app
            .world
            .get_resource_mut::<Events<ProjectileHitEvent>>()
            .unwrap();

        events.send(ProjectileHitEvent { projectile, target });

        app.update();

        let events = app.world.get_resource::<Events<DamageEvent>>().unwrap();
        let mut reader = events.get_reader();
        let event = reader.iter(&events).next().unwrap();

        assert_eq!(
            event.instigator, instigator,
            "Instigator should be equal to specified"
        );
        assert_eq!(event.target, target, "Target should be equal to specified");
        assert_eq!(
            event.damage, FROST_BOLT_DAMAGE,
            "Damage should be equal to frost bolt damage"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_event::<ActivationEvent>()
            .add_event::<ProjectileHitEvent>()
            .add_event::<DamageEvent>()
            .add_state(AppState::InGame)
            .add_plugin(HeadlessRenderPlugin)
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

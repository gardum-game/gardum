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

use super::{HeroBundle, HeroKind};
use crate::{
    characters::{
        ability::{Abilities, AbilitySlot, ActivationEvent},
        cooldown::Cooldown,
        health::DamageEvent,
        projectile::{ProjectileBundle, ProjectileHitEvent},
        CharacterBundle, CharacterOwner,
    },
    core::{player::PlayerOwner, AppState, IconPath},
};

const PROJECTILE_SPEED: f32 = 20.0;
pub const FROST_BOLT_SPAWN_OFFSET: f32 = 4.0;
pub const FROST_BOLT_DAMAGE: u32 = 20;

pub struct NorthPlugin;

impl Plugin for NorthPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_in_stack_update(AppState::InGame)
                .with_system(frost_bolt_system.system())
                .with_system(frost_bolt_hit_system.system()),
        );
    }
}

fn frost_bolt_system(
    mut commands: Commands,
    mut events: EventReader<ActivationEvent>,
    #[cfg(feature = "client")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "client")] mut materials: ResMut<Assets<StandardMaterial>>,
    frost_bolt_query: Query<(), With<FrostBoltAbility>>,
    caster_query: Query<&Transform>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    for event in events
        .iter()
        .filter(|event| frost_bolt_query.get(event.ability).is_ok())
    {
        let camera_transform = camera_query.single().unwrap();
        let caster_transform = caster_query.get(event.caster).unwrap();

        commands
            .spawn_bundle(ProjectileBundle::frost_bolt(
                camera_transform,
                caster_transform,
                #[cfg(feature = "client")]
                &mut meshes,
                #[cfg(feature = "client")]
                &mut materials,
            ))
            .insert(FrostBoltProjectile)
            .insert(CharacterOwner(event.caster));
    }
}

fn frost_bolt_hit_system(
    mut hit_events: EventReader<ProjectileHitEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    query: Query<&CharacterOwner, With<FrostBoltProjectile>>,
) {
    for event in hit_events.iter() {
        if let Ok(character) = query.get(event.projectile) {
            damage_events.send(DamageEvent {
                instigator: character.0,
                target: event.target,
                damage: FROST_BOLT_DAMAGE,
            });
        }
    }
}

#[derive(Bundle)]
pub struct FrostBoltBundle {
    pub kind: FrostBoltAbility,
    pub icon: IconPath,
    pub slot: AbilitySlot,
    pub cooldown: Cooldown,
}

impl Default for FrostBoltBundle {
    fn default() -> Self {
        Self {
            kind: FrostBoltAbility,
            icon: "charaters/heroes/north/frost_bolt.png".into(),
            slot: AbilitySlot::BaseAttack,
            cooldown: Cooldown::from_secs(4),
        }
    }
}

pub struct FrostBoltAbility;
pub struct FrostBoltProjectile;

impl HeroBundle {
    pub fn north(
        player: PlayerOwner,
        transform: Transform,
        commands: &mut Commands,
        #[cfg(feature = "client")] meshes: &mut Assets<Mesh>,
        #[cfg(feature = "client")] materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            player,
            kind: HeroKind::North,
            abilities: Abilities(vec![commands.spawn_bundle(FrostBoltBundle::default()).id()]),
            character: CharacterBundle {
                pbr: PbrBundle {
                    #[cfg(feature = "client")]
                    mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                    #[cfg(feature = "client")]
                    material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                    transform,
                    ..Default::default()
                },
                shape: CollisionShape::Capsule {
                    half_segment: 0.5,
                    radius: 0.5,
                },
                ..Default::default()
            },
        }
    }
}

impl ProjectileBundle {
    fn frost_bolt(
        camera_transform: &Transform,
        caster_transform: &Transform,
        #[cfg(feature = "client")] meshes: &mut Assets<Mesh>,
        #[cfg(feature = "client")] materials: &mut Assets<StandardMaterial>,
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
                #[cfg(feature = "client")]
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                #[cfg(feature = "client")]
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform: Transform {
                    translation: caster_transform.translation
                        + camera_transform.rotation * -Vec3::Z * FROST_BOLT_SPAWN_OFFSET,
                    rotation: camera_transform.rotation
                        * Quat::from_rotation_x(90.0_f32.to_radians()),
                    scale: caster_transform.scale,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

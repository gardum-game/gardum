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

use bevy::{ecs::system::EntityCommands, prelude::*};
use heron::{CollisionEvent, CollisionLayers, CollisionShape, PhysicsLayer, RigidBody};

use super::{
    character::{DamageModifier, HealingModifier},
    cooldown::Cooldown,
    effect::{
        periodic_effect::{PeriodicEffectTimer, PeriodicHealthChange},
        EffectTarget, EffectTimer,
    },
    AppState, AssetCommands, CollisionLayer, TransformBundle,
};

pub(super) struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(pickup_collision_system)
                .with_system(pickup_cooldown_system),
        );
    }
}

fn pickup_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut pickups: Query<(&PickupKind, &mut Cooldown)>,
    children: Query<&Children>,
) {
    for (pickup, character) in collision_events.iter().filter_map(|event| {
        let (layers_1, layers_2) = event.collision_layers();
        if layers_1.groups_bits() == CollisionLayer::Pickup.to_bits()
            && layers_2.groups_bits() == CollisionLayer::Character.to_bits()
        {
            return Some(event.rigid_body_entities());
        }
        if layers_2.groups_bits() == CollisionLayer::Pickup.to_bits()
            && layers_1.groups_bits() == CollisionLayer::Character.to_bits()
        {
            let (character, pickup) = event.rigid_body_entities();
            return Some((pickup, character));
        }
        None
    }) {
        let (pickup_kind, mut cooldown) = pickups.get_mut(pickup).unwrap();
        if !cooldown.finished() {
            continue;
        }
        cooldown.reset();

        match pickup_kind {
            PickupKind::Healing => {
                commands.spawn_bundle(HealingEffectBundle::new(character.into()))
            }
            PickupKind::Rage => commands.spawn_bundle(RageEffectBundle::new(character.into())),
            PickupKind::Speed => commands.spawn_bundle(SpeedEffectBundle::new(character.into())),
        };

        let mesh_child = pickup_child_mesh(pickup, &children);
        commands
            .entity(mesh_child)
            .insert(Visibility { is_visible: false });
    }
}

fn pickup_cooldown_system(
    time: Res<Time>,
    mut cooldowns: Query<(Entity, &mut Cooldown), With<PickupKind>>,
    children: Query<&Children>,
    mut visibility: Query<&mut Visibility>,
) {
    for (pickup, mut cooldown) in cooldowns.iter_mut() {
        cooldown.tick(time.delta());
        if cooldown.just_finished() {
            let child_mesh = pickup_child_mesh(pickup, &children);
            visibility.get_mut(child_mesh).unwrap().is_visible = true;
        }
    }
}

/// Returns children entity with pickup mesh from the specified entity
fn pickup_child_mesh(pickup: Entity, children: &Query<&Children>) -> Entity {
    let mut mesh_entity = pickup;
    // Child entity with mesh located deeply in children hierarchy
    for _ in 0..4 {
        let children = children.get(mesh_entity).unwrap();
        mesh_entity = *children.iter().next().unwrap();
    }
    mesh_entity
}

#[derive(Bundle)]
struct PickupBundle {
    pickup_kind: PickupKind,
    cooldown: Cooldown,
    rigid_body: RigidBody,
    shape: CollisionShape,
    collision_layers: CollisionLayers,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl PickupBundle {
    fn new(pickup_kind: PickupKind, translation: Vec3) -> Self {
        Self {
            pickup_kind,
            cooldown: Cooldown::from_secs(10),
            rigid_body: RigidBody::Sensor,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::new(
                CollisionLayer::Pickup,
                CollisionLayer::Character,
            ),
            transform: Transform::from_translation(translation),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(Bundle)]
struct HealingEffectBundle {
    health_change: PeriodicHealthChange,
    periodic_timer: PeriodicEffectTimer,
    timer: EffectTimer,
    target: EffectTarget,
}

impl HealingEffectBundle {
    fn new(target: EffectTarget) -> Self {
        Self {
            health_change: 10.into(),
            target,
            timer: Timer::from_seconds(4.0, false).into(),
            periodic_timer: PeriodicEffectTimer::default(),
        }
    }
}

#[derive(Bundle)]
struct RageEffectBundle {
    damage_modifier: DamageModifier,
    healing_modifier: HealingModifier,
    timer: EffectTimer,
    target: EffectTarget,
}

impl RageEffectBundle {
    fn new(target: EffectTarget) -> Self {
        Self {
            damage_modifier: 0.2.into(),
            healing_modifier: 0.2.into(),
            timer: Timer::from_seconds(10.0, false).into(),
            target,
        }
    }
}

#[derive(Bundle)]
struct SpeedEffectBundle {
    speed_modifier: HealingModifier,
    timer: EffectTimer,
    target: EffectTarget,
}

impl SpeedEffectBundle {
    fn new(target: EffectTarget) -> Self {
        Self {
            speed_modifier: 0.2.into(),
            timer: Timer::from_seconds(10.0, false).into(),
            target,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub(super) enum PickupKind {
    Speed,
    Rage,
    Healing,
}

impl PickupKind {
    fn asset(&self) -> &str {
        match self {
            PickupKind::Speed => "pickup/lightning.glb#Scene0",
            PickupKind::Rage => "pickup/blood_drop.glb#Scene0",
            PickupKind::Healing => "pickup/cross.glb#Scene0",
        }
    }
}

impl<'w, 's> AssetCommands<'w, 's> {
    pub(super) fn spawn_pickup<'a>(
        &'a mut self,
        pickup_kind: PickupKind,
        translation: Vec3,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut entity_commands = self
            .commands
            .spawn_bundle(PickupBundle::new(pickup_kind, translation));

        entity_commands.with_children(|parent| {
            parent
                .spawn_bundle(TransformBundle::from_transform(
                    Transform::from_translation(Vec3::Y / 2.0),
                ))
                .with_children(|parent| {
                    parent.spawn_scene(self.asset_server.load(pickup_kind.asset()));
                });
            parent.spawn_scene(self.asset_server.load("pickup/platform.glb#Scene0"));
        });

        entity_commands
    }
}

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
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
#[cfg(test)]
use strum::EnumIter;

use super::{
    character::{DamageModifier, HealingModifier, SpeedModifier},
    cooldown::Cooldown,
    effect::{
        periodic_effect::{PeriodicEffectTimer, PeriodicHealthChange},
        EffectTarget, EffectTimer,
    },
    game_state::{GameState, InGameOnly},
    AssociatedAsset, CollisionMask,
};

pub(super) struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::spawn_system.run_in_state(GameState::InGame))
            .add_system(Self::interaction_system.run_in_state(GameState::InGame))
            .add_system(Self::cooldown_system.run_in_state(GameState::InGame));
    }
}

impl PickupPlugin {
    fn spawn_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        spawned_pickups: Query<(Entity, &PickupKind), Added<PickupKind>>,
    ) {
        for (pickup, kind) in spawned_pickups.iter() {
            commands
                .entity(pickup)
                .insert_bundle(LocalPickupBundle::default())
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TransformBundle::from_transform(
                            Transform::from_translation(Vec3::Y / 2.0),
                        ))
                        .with_children(|parent| {
                            parent.spawn_scene(asset_server.load(kind.asset_path()));
                        });
                    parent.spawn_scene(asset_server.load(PLATFORM_PATH));
                });
        }
    }

    fn interaction_system(
        mut commands: Commands,
        children: Query<&Children>,
        mut pickups: Query<
            (Entity, &PickupKind, &mut Cooldown, &CollidingEntities),
            Changed<CollidingEntities>,
        >,
    ) {
        for (pickup, pickup_kind, mut cooldown, collisions) in pickups.iter_mut() {
            let character = match collisions.iter().next() {
                Some(character) => character,
                None => continue,
            };

            if !cooldown.finished() {
                continue;
            }
            cooldown.reset();

            match pickup_kind {
                PickupKind::Healing => {
                    commands.spawn_bundle(HealingEffectBundle::new(character.into()))
                }
                PickupKind::Rage => commands.spawn_bundle(RageEffectBundle::new(character.into())),
                PickupKind::Speed => {
                    commands.spawn_bundle(SpeedEffectBundle::new(character.into()))
                }
            };

            let mesh_child = pickup_child_mesh(pickup, &children);
            commands
                .entity(mesh_child)
                .insert(Visibility { is_visible: false });
        }
    }

    fn cooldown_system(
        time: Res<Time>,
        children: Query<&Children>,
        mut cooldowns: Query<(Entity, &mut Cooldown), With<PickupKind>>,
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
}

/// Returns children entity with pickup mesh from the specified entity.
/// TODO 0.8: Use [`BaseBundle`] that propagates visibility.
fn pickup_child_mesh(pickup: Entity, children: &Query<&Children>) -> Entity {
    let mut mesh_entity = pickup;
    // Child entity with mesh located deeply in children hierarchy
    for _ in 0..4 {
        let children = children.get(mesh_entity).unwrap();
        mesh_entity = *children.iter().next().unwrap();
    }
    mesh_entity
}

/// A component bundle to for pickup.
#[derive(Bundle)]
pub(super) struct PickupBundle {
    pickup_kind: PickupKind,

    #[bundle]
    transform: TransformBundle,
}

impl PickupBundle {
    /// Creates a new [`PickupSpawnBundle`] with `pickup_kind` at `translation`.
    pub(super) fn new(pickup_kind: PickupKind, translation: Vec3) -> Self {
        Self {
            pickup_kind,
            transform: TransformBundle::from_transform(Transform::from_translation(translation)),
        }
    }
}

/// A component bundle that will be inserted automatically after inserting the [`PickupKind`] component.
#[derive(Bundle)]
struct LocalPickupBundle {
    name: Name,
    cooldown: Cooldown,
    sensor: Sensor,
    collider: Collider,
    collision_groups: CollisionGroups,
    colliding_entities: CollidingEntities,
    active_events: ActiveEvents,
    ingame_only: InGameOnly,
}

impl Default for LocalPickupBundle {
    fn default() -> Self {
        Self {
            name: "Pickup".into(),
            cooldown: Cooldown::from_secs(10),
            sensor: Sensor,
            collider: Collider::capsule_y(0.5, 0.5),
            collision_groups: CollisionGroups {
                memberships: CollisionMask::PICKUP.bits(),
                filters: CollisionMask::CHARACTER.bits(),
            },
            colliding_entities: CollidingEntities::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            ingame_only: InGameOnly,
        }
    }
}

/// A component bundle for pickup healing effect.
/// Will be inserted automatically on pickup interation with `PickupKind::Healing`.
#[derive(Bundle)]
struct HealingEffectBundle {
    name: Name,
    health_change: PeriodicHealthChange,
    periodic_timer: PeriodicEffectTimer,
    timer: EffectTimer,
    target: EffectTarget,
}

impl HealingEffectBundle {
    fn new(target: EffectTarget) -> Self {
        Self {
            name: "Healing Effect".into(),
            health_change: 10.into(),
            target,
            timer: Timer::from_seconds(4.0, false).into(),
            periodic_timer: PeriodicEffectTimer::default(),
        }
    }
}

/// A component bundle for pickup rage effect.
/// Will be inserted automatically on pickup interation with `PickupKind::Rage`.
#[derive(Bundle)]
struct RageEffectBundle {
    name: Name,
    damage_modifier: DamageModifier,
    healing_modifier: HealingModifier,
    timer: EffectTimer,
    target: EffectTarget,
}

impl RageEffectBundle {
    fn new(target: EffectTarget) -> Self {
        Self {
            name: "Rage Effect".into(),
            damage_modifier: 0.2.into(),
            healing_modifier: 0.2.into(),
            timer: Timer::from_seconds(10.0, false).into(),
            target,
        }
    }
}

#[derive(Bundle)]
struct SpeedEffectBundle {
    name: Name,
    speed_modifier: SpeedModifier,
    timer: EffectTimer,
    target: EffectTarget,
}

/// A component bundle for pickup speed effect.
/// Will be inserted automatically on pickup interation with `PickupKind::Speed`.
impl SpeedEffectBundle {
    fn new(target: EffectTarget) -> Self {
        Self {
            name: "Speed Effect".into(),
            speed_modifier: 0.2.into(),
            timer: Timer::from_seconds(10.0, false).into(),
            target,
        }
    }
}

/// Type of pickup
#[derive(Component, Clone, Copy)]
#[cfg_attr(test, derive(EnumIter))]
pub(super) enum PickupKind {
    Healing,
    Rage,
    Speed,
}

impl AssociatedAsset for PickupKind {
    fn asset_path(&self) -> &str {
        match self {
            PickupKind::Speed => "pickup/lightning.glb#Scene0",
            PickupKind::Rage => "pickup/blood_drop.glb#Scene0",
            PickupKind::Healing => "pickup/cross.glb#Scene0",
        }
    }
}

const PLATFORM_PATH: &str = "pickup/platform.glb#Scene0";

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::{ecs::system::SystemState, gltf::GltfPlugin, scene::ScenePlugin};
    use strum::IntoEnumIterator;

    use super::*;
    use crate::core::{
        character::CharacterBundle,
        headless::{self, HeadlessRenderPlugin},
    };

    #[test]
    fn pickup_applies_effect() {
        let mut app = App::new();
        app.add_plugin(TestPickupPlugin);

        for pickup_kind in PickupKind::iter() {
            let pickup = app
                .world
                .spawn()
                .insert_bundle(PickupBundle::new(pickup_kind, Vec3::default()))
                .id();

            headless::wait_for_asset_loading(&mut app, pickup_kind.asset_path());
            headless::wait_for_asset_loading(&mut app, PLATFORM_PATH);

            let character = app
                .world
                .spawn()
                .insert_bundle(CharacterBundle::default())
                .id();

            app.update();
            app.update();
            app.update();

            let (effect, target) = match pickup_kind {
                PickupKind::Healing => {
                    app.world
                        .query_filtered::<(Entity, &EffectTarget), With<PeriodicHealthChange>>()
                        .iter(&app.world)
                        .next()
                        .expect("An effect with periodic health change effect should be created")
                }
                PickupKind::Rage => {
                    app.world
                        .query_filtered::<(Entity, &EffectTarget), (With<DamageModifier>, With<HealingModifier>)>().iter(&app.world)
                        .next()
                        .expect("An effect with damage and healing modifiers should be created")
                }
                PickupKind::Speed => {
                    app.world
                        .query_filtered::<(Entity, &EffectTarget), With<SpeedModifier>>().iter(&app.world)
                        .next()
                        .expect("An effect with speed modifier should be created")
                }
            };

            assert_eq!(
                target.0, character,
                "Effect should be applied to the colliding character"
            );

            app.world.entity_mut(character).despawn();
            app.world.entity_mut(effect).despawn();
            app.world.entity_mut(pickup).despawn();
        }
    }

    #[test]
    fn pickup_cooldown() {
        let mut app = App::new();
        app.add_plugin(TestPickupPlugin);

        const PICKUP_KIND: PickupKind = PickupKind::Healing;
        let pickup = app
            .world
            .spawn()
            .insert_bundle(PickupBundle::new(PICKUP_KIND, Vec3::default()))
            .id();

        headless::wait_for_asset_loading(&mut app, PICKUP_KIND.asset_path());
        headless::wait_for_asset_loading(&mut app, PLATFORM_PATH);

        let mut system_state: SystemState<Query<&Children>> = SystemState::new(&mut app.world);
        let children = system_state.get(&app.world);
        let mesh = pickup_child_mesh(pickup, &children);

        app.world
            .entity_mut(mesh)
            .insert(Visibility { is_visible: false });

        let mut cooldown = app.world.get_mut::<Cooldown>(pickup).unwrap();
        cooldown.reset();

        app.world.spawn().insert_bundle(CharacterBundle::default());

        app.update();
        app.update();

        assert_eq!(
            app.world.query::<&EffectTarget>().iter(&app.world).len(),
            0,
            "Effect shouldn't be applied because of cooldown"
        );

        let mut cooldown = app.world.get_mut::<Cooldown>(pickup).unwrap();
        let duration_left = cooldown.duration() - cooldown.elapsed();
        cooldown.tick(duration_left - Duration::from_nanos(1)); // Tick to almost end to trigger just_finished inside the system.

        app.update();

        let visibility = app.world.entity(mesh).get::<Visibility>().unwrap();
        assert!(visibility.is_visible, "Pickup mesh should become visible");
    }

    struct TestPickupPlugin;

    impl Plugin for TestPickupPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(GameState::InGame)
                .add_plugin(HeadlessRenderPlugin)
                .add_plugin(HierarchyPlugin)
                .add_plugin(ScenePlugin)
                .add_plugin(GltfPlugin)
                .add_plugin(TransformPlugin)
                .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugin(PickupPlugin);
        }
    }
}

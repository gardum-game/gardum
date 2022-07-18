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
use std::f32::consts::PI;

use super::Map;
use crate::core::{
    game_state::{GameState, InGameOnly},
    pickup::{PickupBundle, PickupKind},
    session::spawn::SpawnPointBundle,
    AssociatedAsset, CollisionMask,
};

pub(super) struct SkyRoofPlugin;

impl Plugin for SkyRoofPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::spawn_system);
    }
}

impl SkyRoofPlugin {
    fn spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        const PROJECTION: f32 = 45.0;
        commands
            .spawn_bundle(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 30000.0,
                    shadow_projection: OrthographicProjection {
                        left: -PROJECTION,
                        right: PROJECTION,
                        bottom: -PROJECTION,
                        top: PROJECTION,
                        near: -10.0 * PROJECTION,
                        far: 10.0 * PROJECTION,
                        ..Default::default()
                    },
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform {
                    rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4.0, -PI / 4.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(InGameOnly);

        commands.spawn_bundle(SpawnPointBundle::new(Vec3::new(0.0, 5.0, 0.0)));

        commands.spawn_bundle(PickupBundle::new(
            PickupKind::Healing,
            Vec3::new(4.0, 0.1, -1.0),
        ));
        commands.spawn_bundle(PickupBundle::new(
            PickupKind::Speed,
            Vec3::new(4.0, 0.1, 0.0),
        ));
        commands.spawn_bundle(PickupBundle::new(
            PickupKind::Rage,
            Vec3::new(4.0, 0.1, 1.0),
        ));

        let map = asset_server.load(Map::SkyRoof.asset_path());
        commands
            .spawn_bundle(TransformBundle::default())
            .insert(AsyncSceneCollider {
                handle: map.clone(),
                shape: Some(ComputedColliderShape::TriMesh),
                named_shapes: Default::default(),
            })
            .insert(RigidBody::Fixed)
            .insert(CollisionGroups {
                memberships: CollisionMask::WORLD.bits(),
                filters: CollisionMask::all().bits(),
            })
            .insert(InGameOnly)
            .with_children(|parent| {
                parent.spawn_scene(map);
            });
    }
}

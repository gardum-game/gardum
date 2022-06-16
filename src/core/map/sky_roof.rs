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

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

use super::Map;
use crate::core::{
    game_state::InGameOnly, pickup::PickupKind, session::spawn::SpawnPointBundle, AssetCommands,
    AssociatedAsset, CollisionMask,
};

impl<'w, 's> AssetCommands<'w, 's> {
    pub(super) fn spawn_sky_roof<'a>(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        const PROJECTION: f32 = 45.0;
        self.commands
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

        self.commands
            .spawn_bundle(SpawnPointBundle::new(Vec3::new(0.0, 5.0, 0.0)));

        self.spawn_pickup(PickupKind::Healing, Vec3::new(4.0, 0.1, -1.0));
        self.spawn_pickup(PickupKind::Speed, Vec3::new(4.0, 0.1, 0.0));
        self.spawn_pickup(PickupKind::Rage, Vec3::new(4.0, 0.1, 1.0));

        let mut scene_commands = self.commands.spawn_bundle(TransformBundle::default());
        let map = self.asset_server.load(Map::SkyRoof.asset_path());
        scene_commands
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
        scene_commands
    }
}

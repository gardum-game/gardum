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
use heron::{PendingConvexCollision, RigidBody};
use std::f32::consts::PI;

use crate::core::{pickup::PickupKind, session::spawn::SpawnPoint, AssetCommands, TransformBundle};

impl AssetCommands<'_, '_> {
    pub(super) fn spawn_sky_roof(&mut self) {
        const PROJECTION: f32 = 5.0;
        self.commands.spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 100000.0,
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
                translation: Vec3::Y * 5.0,
                rotation: Quat::from_euler(EulerRot::XYZ, -PI / 8.0, -PI / 4.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        self.commands
            .spawn()
            .insert(SpawnPoint(Vec3::new(0.0, 5.0, 0.0)));

        self.commands
            .spawn_bundle(TransformBundle::default())
            .insert(PendingConvexCollision {
                body_type: RigidBody::Static,
                border_radius: None,
            })
            .with_children(|parent| {
                parent.spawn_scene(self.asset_server.load("maps/sky_roof.glb#Scene0"));
            });

        self.spawn_pickup(PickupKind::Healing, Vec3::new(4.0, 0.1, -1.0));
        self.spawn_pickup(PickupKind::Speed, Vec3::new(4.0, 0.1, 0.0));
        self.spawn_pickup(PickupKind::Rage, Vec3::new(4.0, 0.1, 1.0));
    }
}

/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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

use bevy::{input::mouse::MouseMotion, prelude::*, transform::TransformSystem};
use derive_more::{Deref, DerefMut};
use heron::PhysicsSystem;

use crate::core::{AppState, Authority};

pub const CAMERA_DISTANCE: f32 = 10.0;
const CAMERA_SENSETIVITY: f32 = 0.2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(spawn_camera_system.system()),
        )
        .add_system_set(
            SystemSet::on_in_stack_update(AppState::InGame)
                .with_system(camera_input_system.system()),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            camera_position_system
                .system()
                .after(PhysicsSystem::TransformUpdate)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn spawn_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrbitCameraBundle::default())
        .insert(Authority);
}

fn camera_input_system(
    time: Res<Time>,
    mut motion_reader: EventReader<MouseMotion>,
    mut query: Query<&mut OrbitRotation, With<Authority>>,
) {
    let mut orbit_rotation = query.single_mut().unwrap();
    for event in motion_reader.iter() {
        orbit_rotation.0 -= event.delta * CAMERA_SENSETIVITY * time.delta_seconds();
    }

    orbit_rotation.y = orbit_rotation
        .y
        .clamp(10_f32.to_radians(), 90_f32.to_radians());
}

fn camera_position_system(
    app_state: Res<State<AppState>>,
    mut query: QuerySet<(
        Query<&Transform, (With<Authority>, Without<OrbitRotation>)>,
        Query<(&mut Transform, &OrbitRotation), With<Authority>>,
    )>,
) {
    if *app_state.current() != AppState::InGame {
        return;
    }

    let player_translation = match query.q0().single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let (mut camera_transform, orbit_rotation) = query.q1_mut().single_mut().unwrap();
    camera_transform.translation =
        orbit_rotation.to_quat() * Vec3::Y * CAMERA_DISTANCE + player_translation;
    camera_transform.look_at(player_translation, Vec3::Y);
}

#[derive(Bundle, Default)]
pub struct OrbitCameraBundle {
    orbit_rotation: OrbitRotation,

    #[bundle]
    camera: PerspectiveCameraBundle,
}

#[derive(Deref, DerefMut, Debug, PartialEq)]
pub struct OrbitRotation(pub Vec2);

impl OrbitRotation {
    fn to_quat(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.x) * Quat::from_axis_angle(Vec3::X, self.y)
    }
}

impl Default for OrbitRotation {
    fn default() -> Self {
        Self(Vec2::new(0.0, 60_f32.to_radians()))
    }
}

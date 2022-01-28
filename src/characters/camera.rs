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

use bevy::{input::mouse::MouseMotion, prelude::*, transform::TransformSystem};
use derive_more::{Deref, DerefMut};
use heron::PhysicsSystem;

use crate::core::{AppState, Local};

const CAMERA_DISTANCE: f32 = 10.0;
const CAMERA_SENSETIVITY: f32 = 0.2;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_camera_system))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(camera_input_system))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                camera_position_system
                    .after(PhysicsSystem::TransformUpdate)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

fn spawn_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrbitCameraBundle::default())
        .insert(Local);
}

fn camera_input_system(
    time: Res<Time>,
    #[cfg(not(test))] windows: ResMut<Windows>,
    mut motion_reader: EventReader<MouseMotion>,
    mut orbit_rotations: Query<&mut OrbitRotation, With<Local>>,
) {
    #[cfg(not(test))] // Can't run tests with windows, ignore.
    if !windows.get_primary().unwrap().cursor_locked() {
        return;
    }

    let mut orbit_rotation = orbit_rotations.single_mut();
    for event in motion_reader.iter() {
        orbit_rotation.0 -= event.delta * CAMERA_SENSETIVITY * time.delta_seconds();
    }

    orbit_rotation.y = orbit_rotation
        .y
        .clamp(10_f32.to_radians(), 90_f32.to_radians());
}

fn camera_position_system(
    app_state: Res<State<AppState>>,
    character_transforms: Query<&Transform, (With<Local>, Without<OrbitRotation>)>,
    mut cameras: Query<(&mut Transform, &OrbitRotation), With<Local>>,
) {
    if *app_state.current() != AppState::InGame {
        return;
    }

    let character_translation = match character_transforms.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let (mut camera_transform, orbit_rotation) = cameras.single_mut();
    camera_transform.translation =
        orbit_rotation.to_quat() * Vec3::Y * CAMERA_DISTANCE + character_translation;
    camera_transform.look_at(character_translation, Vec3::Y);
}

#[derive(Bundle, Default)]
struct OrbitCameraBundle {
    orbit_rotation: OrbitRotation,

    #[bundle]
    camera: PerspectiveCameraBundle,
}

#[derive(Component, Deref, DerefMut, Debug, PartialEq)]
struct OrbitRotation(Vec2);

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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use bevy::{app::Events, input::InputPlugin};
    use heron::PhysicsPlugin;
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn camera_input() {
        let mut app = setup_app();

        app.update();

        let mut events = app.world.get_resource_mut::<Events<MouseMotion>>().unwrap();
        events.send(MouseMotion { delta: Vec2::ONE });

        app.update();

        let mut orbit_rotations = app.world.query::<&OrbitRotation>();
        let orbit_rotation = orbit_rotations.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
        assert_ne!(
            *orbit_rotation,
            OrbitRotation::default(),
            "Orbital rotation should change after input"
        );
    }

    #[test]
    fn camera_moves_around_character() {
        let mut app = setup_app();
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();

        app.update();

        for (character_translation, camera_rotation) in [
            (Vec3::ZERO, Vec2::ZERO),
            (Vec3::ONE * CAMERA_DISTANCE, Vec2::ZERO),
            (Vec3::ONE, Vec2::ONE * PI),
            (Vec3::ONE, Vec2::ONE * 2.0 * PI),
        ] {
            let mut cameras = app.world.query_filtered::<Entity, With<OrbitRotation>>();
            let camera = cameras.iter(&app.world).next().unwrap(); // TODO 0.7: Use single

            app.world
                .get_mut::<Transform>(character)
                .unwrap()
                .translation = character_translation;
            app.world.get_mut::<OrbitRotation>(camera).unwrap().0 = camera_rotation;

            app.update();

            let camera_transform = app.world.get::<Transform>(camera).unwrap();
            let character_transform = app.world.get::<Transform>(character).unwrap();

            assert_relative_eq!(
                camera_transform
                    .translation
                    .distance(character_transform.translation),
                CAMERA_DISTANCE,
                epsilon = 0.001
            );
            assert_eq!(
                *camera_transform,
                camera_transform.looking_at(character_transform.translation, Vec3::Y),
                "Camera should look at the character"
            );
        }
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(InputPlugin)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(CameraPlugin);
        app
    }

    #[derive(Bundle, Default)]
    struct DummyCharacterBundle {
        transform: Transform,
        local: Local,
    }
}

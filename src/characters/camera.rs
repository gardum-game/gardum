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

use super::CharacterControl;
use crate::core::{AppState, Authority};

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
        .insert(Authority);
}

fn camera_input_system(
    time: Res<Time>,
    character_control: Option<Res<CharacterControl>>,
    mut motion_reader: EventReader<MouseMotion>,
    mut query: Query<&mut OrbitRotation, With<Authority>>,
) {
    if character_control.is_none() {
        return;
    }

    let mut orbit_rotation = query.single_mut();
    for event in motion_reader.iter() {
        orbit_rotation.0 -= event.delta * CAMERA_SENSETIVITY * time.delta_seconds();
    }

    orbit_rotation.y = orbit_rotation
        .y
        .clamp(10_f32.to_radians(), 90_f32.to_radians());
}

fn camera_position_system(
    app_state: Res<State<AppState>>,
    player_query: Query<&Transform, (With<Authority>, Without<OrbitRotation>)>,
    mut camera_query: Query<(&mut Transform, &OrbitRotation), With<Authority>>,
) {
    if *app_state.current() != AppState::InGame {
        return;
    }

    let player_translation = match player_query.get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    let (mut camera_transform, orbit_rotation) = camera_query.single_mut();
    camera_transform.translation =
        orbit_rotation.to_quat() * Vec3::Y * CAMERA_DISTANCE + player_translation;
    camera_transform.look_at(player_translation, Vec3::Y);
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

        let mut query = app.world.query::<&OrbitRotation>();
        let orbit_rotation = query.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
        assert_eq!(
            *orbit_rotation,
            OrbitRotation::default(),
            "Orbital rotation shouldn't change after input without character control"
        );

        app.insert_resource(CharacterControl);

        let mut events = app.world.get_resource_mut::<Events<MouseMotion>>().unwrap();
        events.send(MouseMotion { delta: Vec2::ONE });

        app.update();

        let orbit_rotation = query.iter(&app.world).next().unwrap(); // TODO 0.7: Use single
        assert_ne!(
            *orbit_rotation,
            OrbitRotation::default(),
            "Orbital rotation should change after input"
        );
    }

    #[test]
    fn camera_moves_around_player() {
        let mut app = setup_app();
        app.insert_resource(CharacterControl);
        let player = app
            .world
            .spawn()
            .insert_bundle(DummyPlayerBundle::default())
            .id();

        app.update();

        for (player_translation, camera_rotation) in [
            (Vec3::ZERO, Vec2::ZERO),
            (Vec3::ONE * CAMERA_DISTANCE, Vec2::ZERO),
            (Vec3::ONE, Vec2::ONE * PI),
            (Vec3::ONE, Vec2::ONE * 2.0 * PI),
        ] {
            let mut query = app.world.query_filtered::<Entity, With<OrbitRotation>>();
            let camera = query.iter(&app.world).next().unwrap(); // TODO 0.7: Use single

            app.world.get_mut::<Transform>(player).unwrap().translation = player_translation;
            app.world.get_mut::<OrbitRotation>(camera).unwrap().0 = camera_rotation;

            app.update();

            let camera_transform = app.world.get::<Transform>(camera).unwrap();
            let player_transform = app.world.get::<Transform>(player).unwrap();

            assert_relative_eq!(
                camera_transform
                    .translation
                    .distance(player_transform.translation),
                CAMERA_DISTANCE,
                epsilon = 0.001
            );
            assert_eq!(
                *camera_transform,
                camera_transform.looking_at(player_transform.translation, Vec3::Y),
                "Camera should look at the player"
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
    struct DummyPlayerBundle {
        transform: Transform,
        authority: Authority,
    }
}

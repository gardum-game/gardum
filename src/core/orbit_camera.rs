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

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::{ActiveCamera, Camera3d},
    transform::TransformSystem,
};
use derive_more::From;
use iyes_loopless::prelude::*;

use super::{
    character::hero::HeroKind,
    game_state::{GameState, InGameOnly},
    Authority,
};

const CAMERA_DISTANCE: f32 = 10.0;
const CAMERA_SENSETIVITY: f32 = 0.2;

pub(super) struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::spawn_system.run_in_state(GameState::InGame))
            .add_system(Self::input_system.run_in_state(GameState::InGame))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::position_system
                    .run_in_state(GameState::InGame)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

impl OrbitCameraPlugin {
    fn spawn_system(
        mut commands: Commands,
        mut active_camera: ResMut<ActiveCamera<Camera3d>>,
        spawned_heroes: Query<(Entity, Option<&Authority>), Added<HeroKind>>,
    ) {
        for (hero, authority) in spawned_heroes.iter() {
            let mut entity_commands = commands.spawn_bundle(OrbitCameraBundle::new(hero.into()));

            if authority.is_some() {
                entity_commands.insert(Authority);
                active_camera.set(entity_commands.id());
            }
        }
    }

    fn input_system(
        mut motion_events: EventReader<MouseMotion>,
        time: Res<Time>,
        #[cfg(not(test))] windows: ResMut<Windows>,
        mut orbit_rotations: Query<&mut OrbitRotation, With<Authority>>,
    ) {
        #[cfg(not(test))] // Can't run tests with windows, ignore.
        if !windows.primary().cursor_locked() {
            return;
        }

        if let Ok(mut orbit_rotation) = orbit_rotations.get_single_mut() {
            for event in motion_events.iter() {
                orbit_rotation.0 -= event.delta * CAMERA_SENSETIVITY * time.delta_seconds();
            }

            orbit_rotation.y = orbit_rotation
                .y
                .clamp(10_f32.to_radians(), 90_f32.to_radians());
        }
    }

    fn position_system(
        transforms: Query<&Transform, Without<OrbitRotation>>,
        mut cameras: Query<(&mut Transform, &OrbitRotation, &CameraTarget)>,
    ) {
        for (mut camera_transform, orbit_rotation, target) in cameras.iter_mut() {
            let character_translation = transforms.get(target.0).unwrap().translation;
            camera_transform.translation =
                orbit_rotation.to_quat() * Vec3::Y * CAMERA_DISTANCE + character_translation;
            camera_transform.look_at(character_translation, Vec3::Y);
        }
    }
}

#[derive(Bundle)]
struct OrbitCameraBundle {
    name: Name,
    camera_target: CameraTarget,
    orbit_rotation: OrbitRotation,
    ingame_only: InGameOnly,

    #[bundle]
    camera: PerspectiveCameraBundle<Camera3d>,
}

impl OrbitCameraBundle {
    fn new(camera_target: CameraTarget) -> Self {
        Self {
            name: "Orbit Camera".into(),
            camera_target,
            orbit_rotation: OrbitRotation::default(),
            ingame_only: InGameOnly,
            camera: PerspectiveCameraBundle::new_3d(),
        }
    }
}

#[derive(Component, From)]
pub(super) struct CameraTarget(pub(super) Entity);

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
    use approx::assert_ulps_eq;
    use bevy::{ecs::event::Events, input::InputPlugin, scene::ScenePlugin};
    use bevy_rapier3d::prelude::*;
    use std::f32::consts::PI;

    use super::*;
    use crate::core::headless::HeadlessRenderPlugin;

    #[test]
    fn camera_spawn() {
        let mut app = App::new();
        app.add_plugin(TestOrbitCameraPlugin);

        let local_player = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();

        app.update();

        let active_camera = app.world.resource::<ActiveCamera<Camera3d>>();
        let camera = active_camera.get().expect("3D camera should present");
        let camera_target = app.world.get::<CameraTarget>(camera).unwrap();
        assert_eq!(
            camera_target.0, local_player,
            "Active camera should target local player"
        );

        app.world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .remove::<Authority>();

        app.update();

        let mut cameras = app.world.query::<&Camera>();
        assert_eq!(
            cameras.iter(&app.world).count(),
            2,
            "A new camera should be spawned for new hero"
        );

        let active_camera = app.world.resource::<ActiveCamera<Camera3d>>();
        let current_camera = active_camera.get().expect("3D camera should present");
        assert_eq!(
            camera, current_camera,
            "Active camera should should remain the same because the new hero isn't local"
        );
    }

    #[test]
    fn camera_input() {
        let mut app = App::new();
        app.add_plugin(TestOrbitCameraPlugin);

        app.world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default());

        app.update();

        let mut motion_events = app.world.resource_mut::<Events<MouseMotion>>();
        motion_events.send(MouseMotion { delta: Vec2::ONE });

        app.update();

        let mut orbit_rotations = app.world.query::<&OrbitRotation>();
        let orbit_rotation = orbit_rotations.iter(&app.world).next().unwrap(); // TODO 0.8: Use single
        assert_ne!(
            *orbit_rotation,
            OrbitRotation::default(),
            "Orbital rotation should change after input"
        );
    }

    #[test]
    fn camera_moves_around_character() {
        let mut app = App::new();
        app.add_plugin(TestOrbitCameraPlugin);

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
            let camera = cameras.iter(&app.world).next().unwrap(); // TODO 0.8: Use single

            app.world
                .get_mut::<Transform>(character)
                .unwrap()
                .translation = character_translation;
            app.world.get_mut::<OrbitRotation>(camera).unwrap().0 = camera_rotation;

            app.update();

            let camera_transform = app.world.get::<Transform>(camera).unwrap();
            let character_transform = app.world.get::<Transform>(character).unwrap();

            assert_ulps_eq!(
                camera_transform
                    .translation
                    .distance(character_transform.translation),
                CAMERA_DISTANCE,
            );
            assert_eq!(
                *camera_transform,
                camera_transform.looking_at(character_transform.translation, Vec3::Y),
                "Camera should look at the character"
            );
        }
    }

    struct TestOrbitCameraPlugin;

    impl Plugin for TestOrbitCameraPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(GameState::InGame)
                .add_plugin(HeadlessRenderPlugin)
                .add_plugin(InputPlugin)
                .add_plugin(ScenePlugin)
                .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugin(OrbitCameraPlugin);
        }
    }

    #[derive(Bundle)]
    struct DummyCharacterBundle {
        transform: Transform,
        authority: Authority,
        hero_kind: HeroKind,
    }

    impl Default for DummyCharacterBundle {
        fn default() -> Self {
            Self {
                transform: Transform::default(),
                authority: Authority,
                hero_kind: HeroKind::North,
            }
        }
    }
}

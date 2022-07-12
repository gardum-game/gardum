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

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::{ActiveCamera, Camera3d},
    transform::TransformSystem,
};
use bevy_rapier3d::prelude::*;
use derive_more::From;
use iyes_loopless::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::{
    character::hero::HeroKind,
    game_state::{GameState, InGameOnly},
    Authority, CollisionMask,
};

pub(super) struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::spawn_system.run_in_state(GameState::InGame))
            .add_system(
                Self::input_system
                    .run_in_state(GameState::InGame)
                    .run_if(cursor_locked),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::position_system
                    .run_in_state(GameState::InGame)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

impl OrbitCameraPlugin {
    const MAX_DISTANCE: f32 = 5.0;

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
        mut orbit_rotations: Query<&mut OrbitRotation, With<Authority>>,
    ) {
        if let Ok(mut orbit_rotation) = orbit_rotations.get_single_mut() {
            for event in motion_events.iter() {
                const SENSETIVITY: f32 = 0.2;
                orbit_rotation.0 -= event.delta * SENSETIVITY * time.delta_seconds();
            }

            orbit_rotation.y = orbit_rotation
                .y
                .clamp(10_f32.to_radians(), 90_f32.to_radians());
        }
    }

    fn position_system(
        rapier_ctx: Res<RapierContext>,
        transforms: Query<&Transform, Without<OrbitRotation>>,
        mut cameras: Query<(&mut Transform, &OrbitRotation, &CameraTarget)>,
    ) {
        for (mut camera_transform, orbit_rotation, target) in cameras.iter_mut() {
            let character_translation = transforms.get(target.0).unwrap().translation;
            let look_position = orbit_rotation.look_position() + character_translation;
            let direction = orbit_rotation.direction();
            let max_camera_translation = look_position + direction * Self::MAX_DISTANCE;

            const MARGIN: f32 = 0.5;
            let distance = rapier_ctx
                .cast_ray(
                    character_translation,
                    (max_camera_translation - character_translation).normalize_or_zero(),
                    Self::MAX_DISTANCE,
                    false,
                    QueryFilter::new().groups(InteractionGroups::new(
                        CollisionMask::CHARACTER.bits(),
                        (CollisionMask::all() ^ CollisionMask::CHARACTER).bits(),
                    )),
                )
                .map(|(_, distance)| (distance - MARGIN).max(MARGIN))
                .unwrap_or(Self::MAX_DISTANCE);

            camera_transform.translation = look_position + direction * distance;
            camera_transform.look_at(look_position, Vec3::Y);
        }
    }
}

fn cursor_locked(#[cfg(not(test))] windows: ResMut<Windows>) -> bool {
    #[cfg(not(test))]
    return windows.primary().cursor_locked();
    #[cfg(test)]
    true
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

/// Camera rotation state
#[derive(Component, Deref, DerefMut, Debug, PartialEq)]
struct OrbitRotation(Vec2);

impl OrbitRotation {
    /// Calculate camera direction.
    fn direction(&self) -> Vec3 {
        Quat::from_axis_angle(Vec3::Y, self.x) * Quat::from_axis_angle(Vec3::X, self.y) * Vec3::Y
    }

    /// Calculate the point at which camera is directed relative to the player.
    fn look_position(&self) -> Vec3 {
        const RIGHT_OFFSET: f32 = 1.2;
        const UP_OFFSET: f32 = 1.0;

        // Calculate position on circle around the player using `RIGHT_OFFSET` as radius and add `UP_OFFSET` to this position.
        Vec3::new(
            RIGHT_OFFSET * (self.x + FRAC_PI_2).sin(),
            UP_OFFSET,
            RIGHT_OFFSET * (self.x + FRAC_PI_2).cos(),
        )
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
    fn spawn() {
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

        assert_eq!(
            app.world.query::<&Camera>().iter(&app.world).count(),
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
    fn input() {
        let mut app = App::new();
        app.add_plugin(TestOrbitCameraPlugin);

        app.world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default());

        app.update();

        let mut motion_events = app.world.resource_mut::<Events<MouseMotion>>();
        motion_events.send(MouseMotion { delta: Vec2::ONE });

        app.update();

        let orbit_rotation = app
            .world
            .query::<&OrbitRotation>()
            .iter(&app.world)
            .next()
            .unwrap(); // TODO 0.8: Use single
        assert_ne!(
            *orbit_rotation,
            OrbitRotation::default(),
            "Orbital rotation should change after input"
        );
    }

    #[test]
    fn movement() {
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
            (Vec3::ONE * OrbitCameraPlugin::MAX_DISTANCE, Vec2::ZERO),
            (Vec3::ONE, Vec2::ONE * PI),
            (Vec3::ONE, Vec2::ONE * 2.0 * PI),
        ] {
            let camera = app
                .world
                .query_filtered::<Entity, With<OrbitRotation>>()
                .iter(&app.world)
                .next()
                .unwrap(); // TODO 0.8: Use single

            app.world
                .get_mut::<Transform>(character)
                .unwrap()
                .translation = character_translation;
            app.world.get_mut::<OrbitRotation>(camera).unwrap().0 = camera_rotation;

            app.update();

            let camera = app.world.entity(camera);
            let orbit_rotation = camera.get::<OrbitRotation>().unwrap();
            let camera_transform = camera.get::<Transform>().unwrap();
            let character_transform = app.world.get::<Transform>(character).unwrap();
            let look_position = orbit_rotation.look_position() + character_transform.translation;

            assert_ulps_eq!(
                camera_transform.translation.distance(look_position),
                OrbitCameraPlugin::MAX_DISTANCE,
            );
            assert_eq!(
                *camera_transform,
                camera_transform.looking_at(look_position, Vec3::Y),
                "Camera should look at the character"
            );
        }
    }

    #[test]
    fn collision() {
        let mut app = App::new();
        app.add_plugin(TestOrbitCameraPlugin);

        app.world
            .spawn()
            .insert(Transform::default())
            .insert(Collider::ball(5.0));
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();

        app.update();

        let camera = app
            .world
            .query_filtered::<Entity, With<OrbitRotation>>()
            .iter(&app.world)
            .next()
            .unwrap(); // TODO 0.8: Use single

        let camera = app.world.entity(camera);
        let orbit_rotation = camera.get::<OrbitRotation>().unwrap();
        let camera_transform = camera.get::<Transform>().unwrap();
        let character_transform = app.world.get::<Transform>(character).unwrap();
        let look_position = orbit_rotation.look_position() + character_transform.translation;

        assert!(
            camera_transform.translation.distance(look_position) < OrbitCameraPlugin::MAX_DISTANCE,
            "Camera should collide with the spawned sphere"
        );
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

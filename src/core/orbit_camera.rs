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
use dolly::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    game_state::{GameState, InGameOnly},
    hero::HeroKind,
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
        mut orbit_rotations: Query<&mut OrbitRotation, With<Authority>>,
    ) {
        if let Ok(mut orbit_rotation) = orbit_rotations.get_single_mut() {
            for event in motion_events.iter() {
                const SENSETIVITY: f32 = 0.2;
                orbit_rotation.0 -= event.delta * SENSETIVITY;
            }

            const MAX_ROTATION: f32 = 90.0;
            orbit_rotation.y = orbit_rotation.y.clamp(-MAX_ROTATION, MAX_ROTATION);
        }
    }

    fn position_system(
        rapier_ctx: Res<RapierContext>,
        time: Res<Time>,
        transforms: Query<&Transform, Without<OrbitRotation>>,
        mut cameras: Query<(&mut Transform, &mut OrbitRig, &OrbitRotation, &CameraTarget)>,
    ) {
        for (mut camera_transform, mut orbit_rig, orbit_rotation, target) in cameras.iter_mut() {
            let mut pivot_translation = transforms.get(target.0).unwrap().translation;
            const GROUND_OFFSET: f32 = 1.5;
            pivot_translation.y += GROUND_OFFSET;
            orbit_rig.driver_mut::<Position>().position = pivot_translation;

            let yaw_pitch = orbit_rig.driver_mut::<YawPitch>();
            yaw_pitch.yaw_degrees = orbit_rotation.x;
            yaw_pitch.pitch_degrees = orbit_rotation.y;

            orbit_rig.driver_mut::<Arm>().offset.z = OrbitRig::MAX_DISTANCE;
            let mut calculated_transform = orbit_rig.update(time.delta_seconds());

            let ray_direction = (calculated_transform.position - pivot_translation).normalize();
            const BALL_RADIUS: f32 = 0.5;
            if let Some((_, distance)) = rapier_ctx.cast_shape(
                pivot_translation + ray_direction * BALL_RADIUS,
                Quat::default(),
                ray_direction,
                &Collider::ball(BALL_RADIUS),
                calculated_transform.position.distance(pivot_translation),
                QueryFilter::new().groups(InteractionGroups::new(
                    CollisionMask::CHARACTER.bits(),
                    (CollisionMask::all() ^ CollisionMask::CHARACTER).bits(),
                )),
            ) {
                // Recalculate arm length on collision
                orbit_rig.driver_mut::<Arm>().offset.z = distance.toi;
                calculated_transform = orbit_rig.update(time.delta_seconds());
            }

            camera_transform.translation = calculated_transform.position;
            camera_transform.rotation = calculated_transform.rotation;
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
    orbit_rig: OrbitRig,
    ingame_only: InGameOnly,

    #[bundle]
    camera: PerspectiveCameraBundle<Camera3d>,
}

impl OrbitCameraBundle {
    fn new(camera_target: CameraTarget) -> Self {
        Self {
            name: "Orbit Camera".into(),
            camera_target,
            orbit_rig: OrbitRig::default(),
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
pub(super) struct OrbitRotation(Vec2);

#[derive(Component, From, Deref, DerefMut)]
struct OrbitRig(CameraRig);

impl OrbitRig {
    const MAX_DISTANCE: f32 = 5.0;
}

impl Default for OrbitRig {
    fn default() -> Self {
        Self(
            CameraRig::builder()
                .with(Position::default())
                .with(YawPitch::default())
                .with(Arm::new(Vec3::new(1.5, 0.0, Self::MAX_DISTANCE)))
                .build(),
        )
    }
}

impl Default for OrbitRotation {
    fn default() -> Self {
        Self(Vec2::new(-90.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, input::InputPlugin, scene::ScenePlugin};
    use bevy_rapier3d::prelude::*;

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
    fn position() {
        let mut app = App::new();
        app.add_plugin(TestOrbitCameraPlugin);

        app.world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default());

        app.update();

        let camera = app
            .world
            .query_filtered::<Entity, With<OrbitRotation>>()
            .iter(&app.world)
            .next()
            .unwrap(); // TODO 0.8: Use single

        let mut orbit_rig = app.world.get_mut::<OrbitRig>(camera).unwrap();
        assert_eq!(
            orbit_rig.driver_mut::<Arm>().offset.z,
            OrbitRig::MAX_DISTANCE,
            "Camera should be at the maximum distance when nothing blocks the line of sight"
        );

        app.world
            .spawn()
            .insert(Transform::default())
            .insert(Collider::ball(OrbitRig::MAX_DISTANCE - 1.0));

        app.update();

        let mut orbit_rig = app.world.get_mut::<OrbitRig>(camera).unwrap();
        assert!(
            orbit_rig.driver_mut::<Arm>().offset.z < OrbitRig::MAX_DISTANCE,
            "Camera distance should decrease when there is an obstacle that blocks the line of sight"
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

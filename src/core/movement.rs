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

use approx::abs_diff_eq;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{
    control_actions::ControlAction, game_state::GameState, hero::SpeedModifier,
    orbit_camera::CameraTarget,
};

const MOVE_SPEED: f32 = 10.0;
const MOVEMENT_INTERPOLATION_SPEED: f32 = 6.0;
const AIR_INTERPOLATION_SPEED: f32 = 0.9;
const JUMP_IMPULSE: f32 = 5.0;
const FLOOR_VELOCITY_EPSILON: f32 = 0.05;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::movement_system.run_in_state(GameState::InGame));
    }
}

impl MovementPlugin {
    fn movement_system(
        time: Res<Time>,
        cameras: Query<(&Transform, &CameraTarget)>,
        mut characters: Query<(&SpeedModifier, &ActionState<ControlAction>, &mut Velocity)>,
    ) {
        for (camera_transform, camera_target) in cameras.iter() {
            let (speed_modifier, action_state, mut velocity) =
                characters.get_mut(camera_target.0).unwrap();

            let falling_velocity = velocity.linvel.y; // Save Y velocity to avoid it interpolation
            let on_floor = abs_diff_eq!(falling_velocity, 0.0, epsilon = FLOOR_VELOCITY_EPSILON);
            let interpolation_speed = if on_floor {
                MOVEMENT_INTERPOLATION_SPEED
            } else {
                AIR_INTERPOLATION_SPEED
            };

            let motion = movement_direction(action_state, camera_transform.rotation)
                * MOVE_SPEED
                * speed_modifier.0;
            velocity.linvel = velocity
                .linvel
                .lerp(motion, interpolation_speed * time.delta_seconds());
            velocity.linvel.y = falling_velocity;

            if on_floor && action_state.pressed(ControlAction::Jump) {
                velocity.linvel.y += JUMP_IMPULSE;
            }
        }
    }
}

fn movement_direction(action_state: &ActionState<ControlAction>, rotation: Quat) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if action_state.pressed(ControlAction::Left) {
        direction.x -= 1.0;
    }
    if action_state.pressed(ControlAction::Right) {
        direction.x += 1.0;
    }
    if action_state.pressed(ControlAction::Forward) {
        direction.z -= 1.0;
    }
    if action_state.pressed(ControlAction::Backward) {
        direction.z += 1.0;
    }

    direction = rotation * direction;
    direction.y = 0.0;

    direction.normalize_or_zero()
}

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne, assert_ulps_eq};
    use bevy::scene::ScenePlugin;
    use leafwing_input_manager::prelude::*;

    use super::*;
    use crate::core::{headless::HeadlessRenderPlugin, Authority};

    #[test]
    fn movement_direction_normalization() {
        let mut action_state = ActionState::<ControlAction>::default();
        action_state.press(ControlAction::Forward);
        action_state.press(ControlAction::Right);

        let direction = movement_direction(&action_state, Quat::IDENTITY);
        assert!(direction.is_normalized(), "Should be normalized");
        assert_eq!(direction.y, 0.0, "Shouldn't point up");
    }

    #[test]
    fn movement_direction_compensation() {
        let mut action_state = ActionState::<ControlAction>::default();
        action_state.press(ControlAction::Forward);
        action_state.press(ControlAction::Backward);
        action_state.press(ControlAction::Right);
        action_state.press(ControlAction::Left);

        let direction = movement_direction(&action_state, Quat::IDENTITY);
        assert_eq!(
            direction.x, 0.0,
            "Should be 0 when opposite buttons are pressed"
        );
        assert_eq!(
            direction.z, 0.0,
            "Should be 0 when opposite buttons are pressed"
        );
    }

    #[test]
    fn movement_direction_empty() {
        let action_state = ActionState::<ControlAction>::default();

        let direction = movement_direction(&action_state, Quat::IDENTITY);
        assert_eq!(
            direction,
            Vec3::ZERO,
            "Should be zero when no buttons are pressed"
        );
    }

    #[test]
    fn character_falls() {
        let mut app = App::new();
        app.add_plugin(TestMovementPlugin);

        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::new(character.into()));

        app.update();
        app.update();
        app.update();

        let velocity = app.world.entity(character).get::<Velocity>().unwrap();
        assert_abs_diff_ne!(velocity.linvel.y, 0.0, epsilon = FLOOR_VELOCITY_EPSILON);

        let mut action_state = app
            .world
            .get_mut::<ActionState<ControlAction>>(character)
            .unwrap();
        action_state.press(ControlAction::Jump);
        let previous_translation = app.world.get::<Transform>(character).unwrap().translation;

        app.update();

        assert!(
            previous_translation.y > app.world.get::<Transform>(character).unwrap().translation.y,
            "Character should't be able to jump"
        );
    }

    #[test]
    fn character_standing_on_platform() {
        let mut app = App::new();
        app.add_plugin(TestMovementPlugin);

        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle {
                transform: Transform::from_translation(Vec3::Y * 2.0),
                ..Default::default()
            })
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::new(character.into()));
        app.world.spawn().insert_bundle(DummyPlainBundle::default());

        app.update();
        app.update();
        app.update();

        let velocity = app.world.entity(character).get::<Velocity>().unwrap();
        assert_abs_diff_eq!(velocity.linvel.y, 0.0, epsilon = FLOOR_VELOCITY_EPSILON);

        let mut action_state = app
            .world
            .get_mut::<ActionState<ControlAction>>(character)
            .unwrap();
        action_state.press(ControlAction::Jump);

        app.update();

        assert!(
            DummyCharacterBundle::default().transform.translation.y
                < app.world.get::<Transform>(character).unwrap().translation.y,
            "Character should be able to jump"
        );
    }

    #[test]
    fn character_moves() {
        let mut app = App::new();
        app.add_plugin(TestMovementPlugin);

        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::new(character.into()));

        app.update();

        let test_data = [
            (ControlAction::Forward, -Vec3::Z),
            (ControlAction::Backward, Vec3::Z),
            (ControlAction::Left, -Vec3::X),
            (ControlAction::Right, Vec3::X),
        ];

        for (key, expected_direction) in test_data.iter() {
            let mut action_state = app
                .world
                .get_mut::<ActionState<ControlAction>>(character)
                .unwrap();
            action_state.release_all();
            action_state.press(*key);

            let previous_translation = app.world.get::<Transform>(character).unwrap().translation;

            // Clean previous velocity to avoid interpolation
            app.world.get_mut::<Velocity>(character).unwrap().linvel = Vec3::ZERO;

            app.update();

            let mut direction =
                app.world.get::<Transform>(character).unwrap().translation - previous_translation;
            direction.y = 0.0; // Remove gravity
            direction = direction.normalize();

            assert_ulps_eq!(direction.x, expected_direction.x);
            assert_ulps_eq!(direction.y, expected_direction.y);
        }
    }

    #[test]
    fn speed_modifier_respected() {
        const SPEED_MODIFIER: f32 = 100.0;
        let mut app = App::new();
        app.add_plugin(TestMovementPlugin);

        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle {
                speed_modifier: SPEED_MODIFIER.into(),
                ..Default::default()
            })
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::new(character.into()));

        app.update();

        let mut action_state = app
            .world
            .get_mut::<ActionState<ControlAction>>(character)
            .unwrap();
        action_state.press(ControlAction::Forward);

        app.update();

        let velocity = app.world.entity(character).get::<Velocity>().unwrap();
        let time = app.world.resource::<Time>().delta_seconds();
        assert_ulps_eq!(
            -velocity.linvel.z,
            MOVE_SPEED * SPEED_MODIFIER * MOVEMENT_INTERPOLATION_SPEED * time,
        );
    }

    struct TestMovementPlugin;

    impl Plugin for TestMovementPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(GameState::InGame)
                .add_plugin(HeadlessRenderPlugin)
                .add_plugin(ScenePlugin)
                .add_plugin(InputManagerPlugin::<ControlAction>::default())
                .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugin(MovementPlugin);
        }
    }

    #[derive(Bundle)]
    struct DummyPlainBundle {
        rigid_body: RigidBody,
        collider: Collider,
        transform: Transform,
        global_transform: GlobalTransform,
    }

    impl Default for DummyPlainBundle {
        fn default() -> Self {
            Self {
                rigid_body: RigidBody::Fixed,
                collider: Collider::cuboid(10.0, 1.0, 10.0),
                transform: Transform::default(),
                global_transform: GlobalTransform::default(),
            }
        }
    }

    #[derive(Bundle)]
    struct DummyCharacterBundle {
        speed_modifier: SpeedModifier,
        rigid_body: RigidBody,
        collider: Collider,
        locked_axes: LockedAxes,
        transform: Transform,
        global_transform: GlobalTransform,
        velocity: Velocity,
        action_state: ActionState<ControlAction>,
        authority: Authority,
    }

    impl Default for DummyCharacterBundle {
        fn default() -> Self {
            Self {
                speed_modifier: SpeedModifier::default(),
                rigid_body: RigidBody::Dynamic,
                collider: Collider::capsule_y(0.5, 0.5),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                transform: Transform::default(),
                global_transform: GlobalTransform::default(),
                velocity: Velocity::default(),
                action_state: ActionState::default(),
                authority: Authority,
            }
        }
    }

    #[derive(Bundle)]
    struct DummyCameraBundle {
        camera_target: CameraTarget,
        transform: Transform,
        authority: Authority,
    }

    impl DummyCameraBundle {
        fn new(camera_target: CameraTarget) -> Self {
            Self {
                camera_target,
                transform: Transform::default(),
                authority: Authority,
            }
        }
    }
}

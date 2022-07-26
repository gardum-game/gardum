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
use leafwing_input_manager::prelude::*;
use std::f32::consts::FRAC_PI_2;

use super::{
    control_actions::ControlAction,
    game_state::GameState,
    hero::SpeedModifier,
    orbit_camera::{CameraTarget, OrbitRotation},
    CollisionMask,
};

/// Movement speed multiplier.
const MOVE_SPEED: f32 = 5.0;
/// Jump force for characters.
const JUMP_IMPULSE: f32 = 5.0;
/// Gravity force for characters.
const GRAVITY: f32 = 9.8;
/// Distance at which the characters is considered grounded.
const GROUND_DIST: f32 = 0.01;
/// Max angle at which the characters can walk.
const MAX_WALKING_ANGLE: f32 = 60.0;
/// Max angle at which the characters can jump.
const MAX_JUMP_ANGLE: f32 = 80.0;
/// Distance that the characters can "snap down" vertical steps.
const VERTICAL_SNAP_DISTANCE: f32 = 0.45;
/// Maximum angle that the characters can "snap down" vertical steps.
const VERTICAL_SNAP_ANGLE: f32 = 30.0;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::movement_system.run_in_state(GameState::InGame));
    }
}

impl MovementPlugin {
    fn movement_system(
        mut fall_speed: Local<FallSpeed>,
        mut jump_state: Local<JumpState>,
        time: Res<Time>,
        rapier_ctx: Res<RapierContext>,
        cameras: Query<(&OrbitRotation, &CameraTarget)>,
        mut characters: Query<(
            &SpeedModifier,
            &ActionState<ControlAction>,
            &mut Transform,
            &Collider,
            // &mut Velocity,
        )>,
    ) {
        for (orbit_rotation, camera_target) in cameras.iter() {
            let (speed_modifier, action_state, mut transform, collider) =
                characters.get_mut(camera_target.0).unwrap();

            // Rotate character based on mouse input.
            transform.rotation = Quat::from_rotation_y(orbit_rotation.x.to_radians());

            // Calculate movement based on the rotation
            let mut movement = movement_direction(action_state, transform.rotation)
                * MOVE_SPEED
                * speed_modifier.0
                * time.delta_seconds();

            // Check if the player is falling.
            let ground_hit =
                cast_character(&rapier_ctx, *transform, -Vec3::Y, collider, GROUND_DIST);
            let angle = ground_hit
                .map(|ground_hit| ground_hit.normal1.angle_between(Vec3::Y))
                .unwrap_or_default();

            // If falling, increase falling speed, otherwise stop falling.
            match ground_hit {
                Some(ground_hit) if angle < MAX_WALKING_ANGLE.to_radians() => {
                    movement = movement.reject_from_normalized(ground_hit.normal1); // Project movement onto the plane to walk down slopes smoothly

                    fall_speed.0 = 0.0;
                    jump_state.coyote_time.reset();
                    jump_state.not_sliding_since_jump = true;
                }
                _ => {
                    fall_speed.0 -= GRAVITY * time.delta_seconds();
                    jump_state.coyote_time.tick(time.delta());
                }
            }

            if action_state.just_pressed(ControlAction::Jump) {
                jump_state.active_time.reset();
            }

            // Jump if:
            // 1. Jump button was pressed less then [`JumpState::active_time`] ago.
            // 2. On the ground.
            // 3. Within the ground jump angle.
            // 4. Has not jumped within the jump cooldown time period.
            // 5. Has only jumped once while sliding.
            if !jump_state.active_time.finished()
                && (ground_hit.is_some() || !jump_state.coyote_time.finished())
                && angle < MAX_JUMP_ANGLE.to_radians()
                && jump_state.cooldown.finished()
                && (jump_state.not_sliding_since_jump
                    || (ground_hit.is_some() && angle < MAX_WALKING_ANGLE.to_radians()))
            {
                fall_speed.0 = JUMP_IMPULSE;

                let jump_buffer_duration = jump_state.active_time.duration();
                jump_state.active_time.set_elapsed(jump_buffer_duration);
                jump_state.cooldown.reset();
                jump_state.not_sliding_since_jump = false;
            } else {
                jump_state.cooldown.tick(time.delta());
                jump_state.active_time.tick(time.delta());
            }

            movement.y += fall_speed.0 * time.delta_seconds();
            move_and_slide(&rapier_ctx, collider, &mut transform, movement);

            if ground_hit.is_some() && jump_state.cooldown.finished() {
                snap_down(&rapier_ctx, &mut transform, collider);
            }
        }
    }
}

/// Casts a shape with [`CollisionMask::Character`] interaction group at a constant linear velocity and retrieve the first collider it hits.
fn cast_character(
    rapier_ctx: &RapierContext,
    transform: Transform,
    direction: Vec3,
    collider: &Collider,
    distance: f32,
) -> Option<Toi> {
    rapier_ctx
        .cast_shape(
            transform.translation,
            transform.rotation,
            direction,
            collider,
            distance,
            QueryFilter::new().groups(InteractionGroups::new(
                CollisionMask::CHARACTER.bits(),
                (CollisionMask::all() ^ CollisionMask::CHARACTER).bits(),
            )),
        )
        .map(|(_, toi)| toi)
}

/// Calculate movement vector by the current viewing angle.
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

    direction.normalize_or_zero()
}

/// Moves the body along a vector.
///
/// If the body collides with another, it will slide along the other body rather than stop immediately.
fn move_and_slide(
    rapier_ctx: &RapierContext,
    collider: &Collider,
    transform: &mut Transform,
    mut movement: Vec3,
) {
    const EPSILON: f32 = 0.001;
    const MAX_BOUNCES: usize = 5;

    let mut bounces = 0;
    while bounces < MAX_BOUNCES && movement.length() > EPSILON {
        // Do a cast of the collider to see if an
        // object is hit during this movement bounce.
        let distance = movement.length();
        let hit = match cast_character(
            rapier_ctx,
            *transform,
            movement.normalize_or_zero(),
            collider,
            distance,
        ) {
            Some(hit) => hit,
            None => {
                // If there is no hit, move to desired position.
                transform.translation += movement;
                // Exit as we are done bouncing.
                return;
            }
        };

        // If we are overlapping with something, just exit.
        if hit.status == TOIStatus::Penetrating {
            return;
        }

        let fraction = hit.toi / distance;
        // Set the fraction of remaining movement (minus some small value).
        transform.translation += movement * fraction;
        // Push slightly along normal to stop from getting caught in walls.
        transform.translation += hit.normal1 * EPSILON * 2.0;
        // Decrease remaining movement by fraction of movement remaining.
        movement *= 1.0 - fraction;

        // Only apply angular change if hitting something.
        // Get angle between surface normal and remaining movement.
        let mut angle_between = hit.normal1.angle_between(movement) - FRAC_PI_2;

        // Normalize angle between to be between 0 and 1.
        // 0 means no angle, 1 means 90 degree angle.
        angle_between = angle_between.abs().min(FRAC_PI_2);
        let normalized_angle = angle_between / FRAC_PI_2;

        // Reduce the remaining movement by the remaining movement that ocurred.
        movement *= (1.0 - normalized_angle).sqrt() * 0.9 + 0.1;

        // Rotate the remaining movement to be projected along the plane
        // of the surface hit (emulate pushing against the object).
        let projected =
            movement.reject_from_normalized(hit.normal1).normalize() * movement.length();

        // If projected remaining movement is less than original remaining movement (so if the projection broke
        // due to float operations), then change this to just project along the vertical.
        if projected.length() + EPSILON < movement.length() {
            movement = movement.reject_from_normalized(Vec3::Y).normalize() * movement.length();
        } else {
            movement = projected;
        }

        // Track number of times the character has bounced.
        bounces += 1;
    }
}

/// Snap the player down if they are within a specific distance of the ground.
fn snap_down(rapier_ctx: &RapierContext, transform: &mut Transform, collider: &Collider) {
    if let Some(hit) = cast_character(
        rapier_ctx,
        *transform,
        -Vec3::Y,
        collider,
        VERTICAL_SNAP_DISTANCE,
    ) {
        const MINIMAL_DISTANCE: f32 = 0.002;
        if hit.status != TOIStatus::Penetrating
            && hit.toi > MINIMAL_DISTANCE
            && hit.normal1.angle_between(Vec3::Y) <= VERTICAL_SNAP_ANGLE.to_radians()
        {
            transform.translation.y -= hit.toi - MINIMAL_DISTANCE;
        }
    }
}

#[derive(Default)]
struct FallSpeed(f32);

struct JumpState {
    /// Time in which players can jump after they walk off the edge off a surface.
    coyote_time: Timer,
    /// Jump activation timer.
    /// Allows players to jump before touching the ground for better responsiveness.
    active_time: Timer,
    /// Minimum delay between player jumps.
    cooldown: Timer,
    /// Has the player jumped while sliding?
    not_sliding_since_jump: bool,
}

impl Default for JumpState {
    fn default() -> Self {
        Self {
            coyote_time: Timer::from_seconds(0.05, false),
            active_time: Timer::from_seconds(0.05, false),
            cooldown: Timer::from_seconds(0.25, false),
            not_sliding_since_jump: Default::default(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use approx::{assert_abs_diff_eq, assert_abs_diff_ne, assert_ulps_eq};
//     use bevy::scene::ScenePlugin;
//     use leafwing_input_manager::prelude::*;
//
//     use super::*;
//     use crate::core::{headless::HeadlessRenderPlugin, Authority};
//
//     #[test]
//     fn movement_direction_normalization() {
//         let mut action_state = ActionState::<ControlAction>::default();
//         action_state.press(ControlAction::Forward);
//         action_state.press(ControlAction::Right);
//
//         let direction = movement_direction(&action_state, Quat::IDENTITY);
//         assert!(direction.is_normalized(), "Should be normalized");
//         assert_eq!(direction.y, 0.0, "Shouldn't point up");
//     }
//
//     #[test]
//     fn movement_direction_compensation() {
//         let mut action_state = ActionState::<ControlAction>::default();
//         action_state.press(ControlAction::Forward);
//         action_state.press(ControlAction::Backward);
//         action_state.press(ControlAction::Right);
//         action_state.press(ControlAction::Left);
//
//         let direction = movement_direction(&action_state, Quat::IDENTITY);
//         assert_eq!(
//             direction.x, 0.0,
//             "Should be 0 when opposite buttons are pressed"
//         );
//         assert_eq!(
//             direction.z, 0.0,
//             "Should be 0 when opposite buttons are pressed"
//         );
//     }
//
//     #[test]
//     fn movement_direction_empty() {
//         let action_state = ActionState::<ControlAction>::default();
//
//         let direction = movement_direction(&action_state, Quat::IDENTITY);
//         assert_eq!(
//             direction,
//             Vec3::ZERO,
//             "Should be zero when no buttons are pressed"
//         );
//     }
//
//     #[test]
//     fn character_falls() {
//         let mut app = App::new();
//         app.add_plugin(TestMovementPlugin);
//
//         let character = app
//             .world
//             .spawn()
//             .insert_bundle(DummyCharacterBundle::default())
//             .id();
//         app.world
//             .spawn()
//             .insert_bundle(DummyCameraBundle::new(character.into()));
//
//         app.update();
//         app.update();
//         app.update();
//
//         let velocity = app.world.entity(character).get::<Velocity>().unwrap();
//         assert_abs_diff_ne!(velocity.linvel.y, 0.0, epsilon = FLOOR_VELOCITY_EPSILON);
//
//         let mut action_state = app
//             .world
//             .get_mut::<ActionState<ControlAction>>(character)
//             .unwrap();
//         action_state.press(ControlAction::Jump);
//         let previous_translation = app.world.get::<Transform>(character).unwrap().translation;
//
//         app.update();
//
//         assert!(
//             previous_translation.y > app.world.get::<Transform>(character).unwrap().translation.y,
//             "Character should't be able to jump"
//         );
//     }
//
//     #[test]
//     fn character_standing_on_platform() {
//         let mut app = App::new();
//         app.add_plugin(TestMovementPlugin);
//
//         let character = app
//             .world
//             .spawn()
//             .insert_bundle(DummyCharacterBundle {
//                 transform: Transform::from_translation(Vec3::Y * 2.0),
//                 ..Default::default()
//             })
//             .id();
//         app.world
//             .spawn()
//             .insert_bundle(DummyCameraBundle::new(character.into()));
//         app.world.spawn().insert_bundle(DummyPlainBundle::default());
//
//         app.update();
//         app.update();
//         app.update();
//
//         let velocity = app.world.entity(character).get::<Velocity>().unwrap();
//         assert_abs_diff_eq!(velocity.linvel.y, 0.0, epsilon = FLOOR_VELOCITY_EPSILON);
//
//         let mut action_state = app
//             .world
//             .get_mut::<ActionState<ControlAction>>(character)
//             .unwrap();
//         action_state.press(ControlAction::Jump);
//
//         app.update();
//
//         assert!(
//             DummyCharacterBundle::default().transform.translation.y
//                 < app.world.get::<Transform>(character).unwrap().translation.y,
//             "Character should be able to jump"
//         );
//     }
//
//     #[test]
//     fn character_moves() {
//         let mut app = App::new();
//         app.add_plugin(TestMovementPlugin);
//
//         let character = app
//             .world
//             .spawn()
//             .insert_bundle(DummyCharacterBundle::default())
//             .id();
//         app.world
//             .spawn()
//             .insert_bundle(DummyCameraBundle::new(character.into()));
//
//         app.update();
//
//         let test_data = [
//             (ControlAction::Forward, -Vec3::Z),
//             (ControlAction::Backward, Vec3::Z),
//             (ControlAction::Left, -Vec3::X),
//             (ControlAction::Right, Vec3::X),
//         ];
//
//         for (key, expected_direction) in test_data.iter() {
//             let mut action_state = app
//                 .world
//                 .get_mut::<ActionState<ControlAction>>(character)
//                 .unwrap();
//             action_state.release_all();
//             action_state.press(*key);
//
//             let previous_translation = app.world.get::<Transform>(character).unwrap().translation;
//
//             // Clean previous velocity to avoid interpolation
//             app.world.get_mut::<Velocity>(character).unwrap().linvel = Vec3::ZERO;
//
//             app.update();
//
//             let mut direction =
//                 app.world.get::<Transform>(character).unwrap().translation - previous_translation;
//             direction.y = 0.0; // Remove gravity
//             direction = direction.normalize();
//
//             assert_ulps_eq!(direction.x, expected_direction.x);
//             assert_ulps_eq!(direction.y, expected_direction.y);
//         }
//     }
//
//     #[test]
//     fn speed_modifier_respected() {
//         const SPEED_MODIFIER: f32 = 100.0;
//         let mut app = App::new();
//         app.add_plugin(TestMovementPlugin);
//
//         let character = app
//             .world
//             .spawn()
//             .insert_bundle(DummyCharacterBundle {
//                 speed_modifier: SPEED_MODIFIER.into(),
//                 ..Default::default()
//             })
//             .id();
//         app.world
//             .spawn()
//             .insert_bundle(DummyCameraBundle::new(character.into()));
//
//         app.update();
//
//         let mut action_state = app
//             .world
//             .get_mut::<ActionState<ControlAction>>(character)
//             .unwrap();
//         action_state.press(ControlAction::Forward);
//
//         app.update();
//
//         let velocity = app.world.entity(character).get::<Velocity>().unwrap();
//         let time = app.world.resource::<Time>().delta_seconds();
//         assert_ulps_eq!(
//             -velocity.linvel.z,
//             MOVE_SPEED * SPEED_MODIFIER * MOVEMENT_INTERPOLATION_SPEED * time,
//         );
//     }
//
//     struct TestMovementPlugin;
//
//     impl Plugin for TestMovementPlugin {
//         fn build(&self, app: &mut App) {
//             app.add_loopless_state(GameState::InGame)
//                 .add_plugin(HeadlessRenderPlugin)
//                 .add_plugin(ScenePlugin)
//                 .add_plugin(InputManagerPlugin::<ControlAction>::default())
//                 .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
//                 .add_plugin(MovementPlugin);
//         }
//     }
//
//     #[derive(Bundle)]
//     struct DummyPlainBundle {
//         rigid_body: RigidBody,
//         collider: Collider,
//         transform: Transform,
//         global_transform: GlobalTransform,
//     }
//
//     impl Default for DummyPlainBundle {
//         fn default() -> Self {
//             Self {
//                 rigid_body: RigidBody::Fixed,
//                 collider: Collider::cuboid(10.0, 1.0, 10.0),
//                 transform: Transform::default(),
//                 global_transform: GlobalTransform::default(),
//             }
//         }
//     }
//
//     #[derive(Bundle)]
//     struct DummyCharacterBundle {
//         speed_modifier: SpeedModifier,
//         rigid_body: RigidBody,
//         collider: Collider,
//         locked_axes: LockedAxes,
//         transform: Transform,
//         global_transform: GlobalTransform,
//         velocity: Velocity,
//         action_state: ActionState<ControlAction>,
//         authority: Authority,
//     }
//
//     impl Default for DummyCharacterBundle {
//         fn default() -> Self {
//             Self {
//                 speed_modifier: SpeedModifier::default(),
//                 rigid_body: RigidBody::Dynamic,
//                 collider: Collider::capsule_y(0.5, 0.5),
//                 locked_axes: LockedAxes::ROTATION_LOCKED,
//                 transform: Transform::default(),
//                 global_transform: GlobalTransform::default(),
//                 velocity: Velocity::default(),
//                 action_state: ActionState::default(),
//                 authority: Authority,
//             }
//         }
//     }
//
//     #[derive(Bundle)]
//     struct DummyCameraBundle {
//         camera_target: CameraTarget,
//         transform: Transform,
//         authority: Authority,
//     }
//
//     impl DummyCameraBundle {
//         fn new(camera_target: CameraTarget) -> Self {
//             Self {
//                 camera_target,
//                 transform: Transform::default(),
//                 authority: Authority,
//             }
//         }
//     }
// }

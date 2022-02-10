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
use heron::{rapier_plugin::PhysicsWorld, CollisionLayers, CollisionShape, Velocity};
use leafwing_input_manager::prelude::ActionState;

use super::{
    character::SpeedModifier, character_action::CharacterAction, orbit_camera::CameraTarget,
    AppState,
};

const MOVE_SPEED: f32 = 10.0;
const GRAVITY: f32 = 9.8;
const VELOCITY_INTERPOLATE_SPEED: f32 = 6.0;
const JUMP_IMPULSE: f32 = 25.0;
const FLOOR_THRESHOLD: f32 = 0.01;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(movement_system));
    }
}

fn movement_system(
    time: Res<Time>,
    physics_world: PhysicsWorld,
    cameras: Query<(&Transform, &CameraTarget)>,
    mut characters: Query<(
        Entity,
        &SpeedModifier,
        &ActionState<CharacterAction>,
        &Transform,
        &CollisionShape,
        &mut Velocity,
    )>,
) {
    for (camera_transform, camera_target) in cameras.iter() {
        let (character, speed_modifier, actions, transform, shape, mut velocity) =
            characters.get_mut(camera_target.0).unwrap();

        let motion =
            movement_direction(actions, camera_transform.rotation) * MOVE_SPEED * speed_modifier.0;
        velocity.linear = velocity
            .linear
            .lerp(motion, VELOCITY_INTERPOLATE_SPEED * time.delta_seconds());

        if is_on_floor(&physics_world, character, shape, transform) {
            if actions.pressed(&CharacterAction::Jump) {
                velocity.linear.y += JUMP_IMPULSE;
            } else {
                velocity.linear.y = 0.0;
            }
        } else {
            velocity.linear.y -= GRAVITY * VELOCITY_INTERPOLATE_SPEED * time.delta_seconds();
        }
    }
}

fn movement_direction(actions: &ActionState<CharacterAction>, rotation: Quat) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if actions.pressed(&CharacterAction::Left) {
        direction.x -= 1.0;
    }
    if actions.pressed(&CharacterAction::Right) {
        direction.x += 1.0;
    }
    if actions.pressed(&CharacterAction::Forward) {
        direction.z -= 1.0;
    }
    if actions.pressed(&CharacterAction::Backward) {
        direction.z += 1.0;
    }

    direction = rotation * direction;
    direction.y = 0.0;

    direction.normalize_or_zero()
}

fn is_on_floor(
    physics_world: &PhysicsWorld,
    entity: Entity,
    shape: &CollisionShape,
    transform: &Transform,
) -> bool {
    physics_world
        .shape_cast_with_filter(
            shape,
            transform.translation,
            transform.rotation,
            -Vec3::X * FLOOR_THRESHOLD,
            CollisionLayers::default(),
            |hit_entity| entity != hit_entity,
        )
        .is_some()
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use bevy::{ecs::system::SystemState, input::InputPlugin};
    use heron::{PhysicsPlugin, RigidBody};
    use leafwing_input_manager::prelude::InputManagerPlugin;

    use super::*;
    use crate::core::Local;

    #[test]
    fn movement_direction_normalization() {
        let mut actions = ActionState::<CharacterAction>::default();
        actions.press(&CharacterAction::Forward);
        actions.press(&CharacterAction::Right);

        let direction = movement_direction(&actions, Quat::IDENTITY);
        assert!(direction.is_normalized(), "Should be normalized");
        assert_eq!(direction.y, 0.0, "Shouldn't point up");
    }

    #[test]
    fn movement_direction_compensation() {
        let mut actions = ActionState::<CharacterAction>::default();
        actions.press(&CharacterAction::Forward);
        actions.press(&CharacterAction::Backward);
        actions.press(&CharacterAction::Right);
        actions.press(&CharacterAction::Left);

        let direction = movement_direction(&actions, Quat::IDENTITY);
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
        let actions = ActionState::<CharacterAction>::default();

        let direction = movement_direction(&actions, Quat::IDENTITY);
        assert_eq!(
            direction,
            Vec3::ZERO,
            "Should be zero when no buttons are pressed"
        );
    }

    #[test]
    fn character_falls() {
        let mut app = setup_app();
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

        // Clone collision and transform because PhysicsWorld is a mutable SystemParam
        let collision_shape = app.world.get::<CollisionShape>(character).unwrap().clone();
        let transform = app.world.get::<Transform>(character).unwrap().clone();
        let mut system_state: SystemState<PhysicsWorld> = SystemState::new(&mut app.world);
        let physics_world = system_state.get_mut(&mut app.world);

        assert!(
            !is_on_floor(&physics_world, character, &collision_shape, &transform,),
            "Character shouldn't be on floor"
        );
        assert!(
            DummyCharacterBundle::default().transform.translation.y > transform.translation.y,
            "Character should be affected by gravity"
        );

        let mut actions = app
            .world
            .get_mut::<ActionState<CharacterAction>>(character)
            .unwrap();
        actions.press(&CharacterAction::Jump);
        let previous_translation = app.world.get::<Transform>(character).unwrap().translation;

        app.update();

        assert!(
            previous_translation.y > app.world.get::<Transform>(character).unwrap().translation.y,
            "Character should't be able to jump"
        );
    }

    #[test]
    fn character_standing_on_platform() {
        let mut app = setup_app();
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::new(character.into()));
        app.world.spawn().insert_bundle(DummyPlainBundle::default());

        app.update();

        let previous_translation = app.world.get::<Transform>(character).unwrap().translation;

        app.update();

        // Clone collision and transform because PhysicsWorld is a mutable SystemParam
        let collision_shape = app.world.get::<CollisionShape>(character).unwrap().clone();
        let transform = app.world.get::<Transform>(character).unwrap().clone();
        let mut system_state: SystemState<PhysicsWorld> = SystemState::new(&mut app.world);
        let physics_world = system_state.get_mut(&mut app.world);

        assert!(
            is_on_floor(&physics_world, character, &collision_shape, &transform,),
            "Character should be on floor"
        );
        assert_eq!(
            previous_translation.y, transform.translation.y,
            "Character shouldn't be affected by gravity"
        );

        let mut actions = app
            .world
            .get_mut::<ActionState<CharacterAction>>(character)
            .unwrap();
        actions.press(&CharacterAction::Jump);

        app.update();

        assert!(
            DummyCharacterBundle::default().transform.translation.y
                < app.world.get::<Transform>(character).unwrap().translation.y,
            "Character should be able to jump"
        );
    }

    #[test]
    fn character_moves() {
        let mut app = setup_app();
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
            (CharacterAction::Forward, -Vec3::Z),
            (CharacterAction::Backward, Vec3::Z),
            (CharacterAction::Left, -Vec3::X),
            (CharacterAction::Right, Vec3::X),
        ];

        for (key, expected_direction) in test_data.iter() {
            let mut actions = app
                .world
                .get_mut::<ActionState<CharacterAction>>(character)
                .unwrap();
            actions.release_all();
            actions.press(key);

            let previous_translation = app
                .world
                .get::<Transform>(character)
                .unwrap()
                .translation
                .clone();

            // Clean previous velocity to avoid interpolation
            app.world.get_mut::<Velocity>(character).unwrap().linear = Vec3::ZERO;

            app.update();

            let mut direction =
                app.world.get::<Transform>(character).unwrap().translation - previous_translation;
            direction.y = 0.0; // Remove gravity
            direction = direction.normalize();

            assert_relative_eq!(direction.x, expected_direction.x);
            assert_relative_eq!(direction.y, expected_direction.y);
        }
    }

    #[test]
    fn speed_modifier_respected() {
        const SPEED_MODIFIER: f32 = 100.0;
        let mut app = setup_app();
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle {
                speed_modifier: SpeedModifier(SPEED_MODIFIER),
                ..Default::default()
            })
            .id();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::new(character.into()));

        app.update();

        let mut actions = app
            .world
            .get_mut::<ActionState<CharacterAction>>(character)
            .unwrap();
        actions.press(&CharacterAction::Forward);

        app.update();

        let time = app.world.get_resource::<Time>().unwrap().delta_seconds();
        let distance = app.world.get::<Transform>(character).unwrap().translation.z;
        assert_relative_eq!(
            distance.abs() / time / MOVE_SPEED / VELOCITY_INTERPOLATE_SPEED / SPEED_MODIFIER,
            time,
        )
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(InputPlugin)
            .add_plugin(InputManagerPlugin::<CharacterAction>::default())
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(InputPlugin)
            .add_plugin(MovementPlugin);
        app
    }

    #[derive(Bundle)]
    struct DummyPlainBundle {
        rigid_body: RigidBody,
        shape: CollisionShape,
        transform: Transform,
        global_transform: GlobalTransform,
    }

    impl Default for DummyPlainBundle {
        fn default() -> Self {
            Self {
                rigid_body: RigidBody::Static,
                shape: CollisionShape::Cuboid {
                    half_extends: Vec3::new(10.0, 0.1, 10.0),
                    border_radius: None,
                },
                transform: Transform::default(),
                global_transform: GlobalTransform::default(),
            }
        }
    }

    #[derive(Bundle)]
    struct DummyCharacterBundle {
        speed_modifier: SpeedModifier,
        rigid_body: RigidBody,
        shape: CollisionShape,
        transform: Transform,
        global_transform: GlobalTransform,
        velocity: Velocity,
        action_state: ActionState<CharacterAction>,
        local: Local,
    }

    impl Default for DummyCharacterBundle {
        fn default() -> Self {
            Self {
                speed_modifier: SpeedModifier::default(),
                rigid_body: RigidBody::KinematicVelocityBased,
                shape: CollisionShape::Capsule {
                    half_segment: 0.5,
                    radius: 0.5,
                },
                transform: Transform::default(),
                global_transform: GlobalTransform::default(),
                velocity: Velocity::default(),
                action_state: ActionState::default(),
                local: Local,
            }
        }
    }

    #[derive(Bundle)]
    struct DummyCameraBundle {
        camera_target: CameraTarget,
        transform: Transform,
        local: Local,
    }

    impl DummyCameraBundle {
        fn new(camera_target: CameraTarget) -> Self {
            Self {
                camera_target,
                transform: Transform::default(),
                local: Local::default(),
            }
        }
    }
}

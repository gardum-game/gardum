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

use bevy::{prelude::*, render::camera::Camera};
use heron::{rapier_plugin::PhysicsWorld, CollisionLayers, CollisionShape, Velocity};
use leafwing_input_manager::prelude::ActionState;

use super::action::Action;
use crate::core::{AppState, Local};

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
    local_camera: Query<&Transform, (With<Camera>, With<Local>)>,
    mut local_character: Query<
        (
            Entity,
            &ActionState<Action>,
            &Transform,
            &CollisionShape,
            &mut Velocity,
        ),
        With<Local>,
    >,
) {
    if let Ok((character, actions, transform, shape, mut velocity)) =
        local_character.get_single_mut()
    {
        let motion = movement_direction(actions, local_camera.single().rotation) * MOVE_SPEED;
        velocity.linear = velocity
            .linear
            .lerp(motion, VELOCITY_INTERPOLATE_SPEED * time.delta_seconds());

        if is_on_floor(&physics_world, character, shape, transform) {
            if actions.pressed(Action::Jump) {
                velocity.linear.y += JUMP_IMPULSE;
            } else {
                velocity.linear.y = 0.0;
            }
        } else {
            velocity.linear.y -= GRAVITY * VELOCITY_INTERPOLATE_SPEED * time.delta_seconds();
        }
    }
}

fn movement_direction(actions: &ActionState<Action>, rotation: Quat) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if actions.pressed(Action::Left) {
        direction.x -= 1.0;
    }
    if actions.pressed(Action::Right) {
        direction.x += 1.0;
    }
    if actions.pressed(Action::Forward) {
        direction.z -= 1.0;
    }
    if actions.pressed(Action::Backward) {
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

    #[test]
    fn movement_direction_normalization() {
        let mut actions = ActionState::<Action>::default();
        actions.press(Action::Forward);
        actions.press(Action::Right);

        let direction = movement_direction(&actions, Quat::IDENTITY);
        assert!(direction.is_normalized(), "Should be normalized");
        assert_eq!(direction.y, 0.0, "Shouldn't point up");
    }

    #[test]
    fn movement_direction_compensation() {
        let mut actions = ActionState::<Action>::default();
        actions.press(Action::Forward);
        actions.press(Action::Backward);
        actions.press(Action::Right);
        actions.press(Action::Left);

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
        let actions = ActionState::<Action>::default();

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
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default());
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();

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

        let mut actions = app.world.get_mut::<ActionState<Action>>(character).unwrap();
        actions.press(Action::Jump);
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
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default());
        app.world.spawn().insert_bundle(DummyPlainBundle::default());
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();

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

        let mut actions = app.world.get_mut::<ActionState<Action>>(character).unwrap();
        actions.press(Action::Jump);

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
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default());
        let character = app
            .world
            .spawn()
            .insert_bundle(DummyCharacterBundle::default())
            .id();

        app.update();

        let test_data = [
            (Action::Forward, -Vec3::Z),
            (Action::Backward, Vec3::Z),
            (Action::Left, -Vec3::X),
            (Action::Right, Vec3::X),
        ];

        for (key, expected_direction) in test_data.iter() {
            let mut actions = app.world.get_mut::<ActionState<Action>>(character).unwrap();
            actions.release_all();
            actions.press(*key);

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

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(InputPlugin)
            .add_plugin(InputManagerPlugin::<Action>::default())
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
        rigid_body: RigidBody,
        shape: CollisionShape,
        transform: Transform,
        global_transform: GlobalTransform,
        velocity: Velocity,
        action_state: ActionState<Action>,
        local: Local,
    }

    impl Default for DummyCharacterBundle {
        fn default() -> Self {
            Self {
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

    #[derive(Bundle, Default)]
    struct DummyCameraBundle {
        camera: Camera,
        transform: Transform,
        local: Local,
    }
}

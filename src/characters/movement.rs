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

use crate::core::{AppState, Authority};

const MOVE_SPEED: f32 = 10.0;
const GRAVITY: f32 = 9.8;
const VELOCITY_INTERPOLATE_SPEED: f32 = 6.0;
const JUMP_IMPULSE: f32 = 25.0;
const FLOOR_THRESHOLD: f32 = 0.01;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementInput>()
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame)
                    .label(MovementSystems::InputSet)
                    .with_system(input_system),
            )
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame)
                    .after(MovementSystems::InputSet)
                    .with_system(movement_system),
            );
    }
}

fn input_system(keys: Res<Input<KeyCode>>, mut input: ResMut<MovementInput>) {
    input.forward = keys.pressed(KeyCode::W);
    input.backward = keys.pressed(KeyCode::S);
    input.left = keys.pressed(KeyCode::A);
    input.right = keys.pressed(KeyCode::D);

    input.jumping = keys.pressed(KeyCode::Space);
}

fn movement_system(
    time: Res<Time>,
    input: Res<MovementInput>,
    physics_world: PhysicsWorld,
    camera_query: Query<&Transform, (With<Camera>, With<Authority>)>,
    mut player_query: Query<(Entity, &Transform, &CollisionShape, &mut Velocity), With<Authority>>,
) {
    if let Ok((entity, transform, shape, mut velocity)) = player_query.get_single_mut() {
        let motion = input.movement_direction(camera_query.single().rotation) * MOVE_SPEED;
        velocity.linear = velocity
            .linear
            .lerp(motion, VELOCITY_INTERPOLATE_SPEED * time.delta_seconds());

        if is_on_floor(&physics_world, entity, shape, transform) {
            if input.jumping {
                velocity.linear.y += JUMP_IMPULSE;
            } else {
                velocity.linear.y = 0.0;
            }
        } else {
            velocity.linear.y -= GRAVITY * VELOCITY_INTERPOLATE_SPEED * time.delta_seconds();
        }
    }
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

#[derive(Default, Debug, PartialEq)]
struct MovementInput {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    jumping: bool,
}

impl MovementInput {
    fn movement_direction(&self, rotation: Quat) -> Vec3 {
        let mut direction = Vec3::ZERO;
        if self.left {
            direction.x -= 1.0;
        }
        if self.right {
            direction.x += 1.0;
        }
        if self.forward {
            direction.z -= 1.0;
        }
        if self.backward {
            direction.z += 1.0;
        }

        direction = rotation * direction;
        direction.y = 0.0;

        direction.normalize_or_zero()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MovementSystems {
    InputSet,
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use bevy::{
        app::Events,
        ecs::system::SystemState,
        input::{keyboard::KeyboardInput, ElementState, InputPlugin},
    };
    use heron::{PhysicsPlugin, RigidBody};

    use super::*;

    #[test]
    fn movement_direction_normalization() {
        let input = MovementInput {
            forward: true,
            backward: false,
            left: true,
            right: false,
            jumping: true,
        };

        let direction = input.movement_direction(Quat::IDENTITY);
        assert!(direction.is_normalized(), "Should be normalized");
        assert_eq!(direction.y, 0.0, "Shouldn't point up");
    }

    #[test]
    fn movement_direction_compensation() {
        let input = MovementInput {
            forward: true,
            backward: true,
            left: true,
            right: true,
            jumping: true,
        };

        let direction = input.movement_direction(Quat::IDENTITY);
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
        let input = MovementInput {
            forward: false,
            backward: false,
            left: false,
            right: false,
            jumping: false,
        };

        let direction = input.movement_direction(Quat::IDENTITY);
        assert_eq!(
            direction,
            Vec3::ZERO,
            "Should be zero when no buttons are pressed"
        );
    }

    #[test]
    fn movement_input() {
        let mut app = setup_app();
        app.update();

        let test_data = [
            (
                KeyCode::W,
                MovementInput {
                    forward: true,
                    ..Default::default()
                },
            ),
            (
                KeyCode::S,
                MovementInput {
                    backward: true,
                    ..Default::default()
                },
            ),
            (
                KeyCode::A,
                MovementInput {
                    left: true,
                    ..Default::default()
                },
            ),
            (
                KeyCode::D,
                MovementInput {
                    right: true,
                    ..Default::default()
                },
            ),
        ];

        for (i, (key, expected_input)) in test_data.iter().enumerate() {
            let mut events = app
                .world
                .get_resource_mut::<Events<KeyboardInput>>()
                .unwrap();

            if i != 0 {
                // Previous key should be released manually
                events.send(KeyboardInput {
                    scan_code: 0,
                    key_code: Some(test_data[i - 1].0),
                    state: ElementState::Released,
                });
            }

            events.send(KeyboardInput {
                scan_code: 0,
                key_code: Some(*key),
                state: ElementState::Pressed,
            });

            app.update();

            assert_eq!(
                app.world.get_resource::<MovementInput>().unwrap(),
                expected_input,
                "Movement input should correspond to the pressed key: {:?}",
                key
            );
        }
    }

    #[test]
    fn player_falls() {
        let mut app = setup_app();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default());
        let player = app
            .world
            .spawn()
            .insert_bundle(DummyPlayerBundle::default())
            .id();

        app.update();
        app.update();

        // Clone collision and transform because PhysicsWorld is a mutable SystemParam
        let collision_shape = app.world.get::<CollisionShape>(player).unwrap().clone();
        let transform = app.world.get::<Transform>(player).unwrap().clone();
        let mut system_state: SystemState<PhysicsWorld> = SystemState::new(&mut app.world);
        let physics_world = system_state.get_mut(&mut app.world);

        assert!(
            !is_on_floor(&physics_world, player, &collision_shape, &transform,),
            "Player shouldn't be on floor"
        );
        assert!(
            DummyPlayerBundle::default().transform.translation.y > transform.translation.y,
            "Player should be affected by gravity"
        );

        let mut events = app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();

        events.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(KeyCode::Space),
            state: ElementState::Pressed,
        });

        let previous_translation = app.world.get::<Transform>(player).unwrap().translation;

        app.update();

        assert!(
            previous_translation.y > app.world.get::<Transform>(player).unwrap().translation.y,
            "Player should't be able to jump"
        );
    }

    #[test]
    fn player_standing_on_platform() {
        let mut app = setup_app();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default());
        app.world.spawn().insert_bundle(DummyPlainBundle::default());
        let player = app
            .world
            .spawn()
            .insert_bundle(DummyPlayerBundle::default())
            .id();

        app.update();

        let previous_translation = app.world.get::<Transform>(player).unwrap().translation;

        app.update();

        // Clone collision and transform because PhysicsWorld is a mutable SystemParam
        let collision_shape = app.world.get::<CollisionShape>(player).unwrap().clone();
        let transform = app.world.get::<Transform>(player).unwrap().clone();
        let mut system_state: SystemState<PhysicsWorld> = SystemState::new(&mut app.world);
        let physics_world = system_state.get_mut(&mut app.world);

        assert!(
            is_on_floor(&physics_world, player, &collision_shape, &transform,),
            "Player should be on floor"
        );
        assert_eq!(
            previous_translation.y, transform.translation.y,
            "Player shouldn't be affected by gravity"
        );

        let mut events = app
            .world
            .get_resource_mut::<Events<KeyboardInput>>()
            .unwrap();

        events.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(KeyCode::Space),
            state: ElementState::Pressed,
        });

        app.update();

        assert!(
            DummyPlayerBundle::default().transform.translation.y
                < app.world.get::<Transform>(player).unwrap().translation.y,
            "Player should be able to jump"
        );
    }

    #[test]
    fn player_moves() {
        let mut app = setup_app();
        app.world
            .spawn()
            .insert_bundle(DummyCameraBundle::default());
        let player = app
            .world
            .spawn()
            .insert_bundle(DummyPlayerBundle::default())
            .id();

        app.update();

        let test_data = [
            (KeyCode::W, -Vec3::Z),
            (KeyCode::S, Vec3::Z),
            (KeyCode::A, -Vec3::X),
            (KeyCode::D, Vec3::X),
        ];

        for (i, (key, expected_direction)) in test_data.iter().enumerate() {
            let mut events = app
                .world
                .get_resource_mut::<Events<KeyboardInput>>()
                .unwrap();

            if i != 0 {
                // Previous key should be released manually
                events.send(KeyboardInput {
                    scan_code: 0,
                    key_code: Some(test_data[i - 1].0),
                    state: ElementState::Released,
                });
            }

            events.send(KeyboardInput {
                scan_code: 0,
                key_code: Some(*key),
                state: ElementState::Pressed,
            });

            let previous_translation = app
                .world
                .get::<Transform>(player)
                .unwrap()
                .translation
                .clone();

            // Clean previous velocity to avoid interpolation
            app.world.get_mut::<Velocity>(player).unwrap().linear = Vec3::ZERO;

            app.update();

            let mut direction =
                app.world.get::<Transform>(player).unwrap().translation - previous_translation;
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
            .add_plugin(PhysicsPlugin::default())
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
    struct DummyPlayerBundle {
        rigid_body: RigidBody,
        shape: CollisionShape,
        transform: Transform,
        global_transform: GlobalTransform,
        velocity: Velocity,
        authority: Authority,
    }

    impl Default for DummyPlayerBundle {
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
                authority: Authority,
            }
        }
    }

    #[derive(Bundle, Default)]
    struct DummyCameraBundle {
        camera: Camera,
        transform: Transform,
        authority: Authority,
    }
}

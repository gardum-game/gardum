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

pub struct MovementPlugin;

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

pub fn is_on_floor(
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
pub struct MovementInput {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jumping: bool,
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
}

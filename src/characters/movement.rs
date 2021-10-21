/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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
use bevy_rapier3d::prelude::{
    ColliderHandle, ColliderShape, InteractionGroups, Isometry, QueryPipeline,
    QueryPipelineColliderComponentsQuery, QueryPipelineColliderComponentsSet, Real,
    RigidBodyColliders, RigidBodyPosition, RigidBodyVelocity, Shape, Vector,
};

use crate::core::{AppState, Authority};

use super::PreviousVelocity;

const MOVE_SPEED: f32 = 10.0;
const GRAVITY: f32 = 9.8;
const VELOCITY_INTERPOLATE_SPEED: f32 = 6.0;
const JUMP_IMPULSE: f32 = 25.0;
const FLOOR_THRESHOLD: f32 = 0.01;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MovementInput>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("input")
                    .with_system(input_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after("input")
                    .with_system(movement_system.system()),
            );
    }
}

#[derive(Default)]
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
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    camera_query: Query<&Transform, (With<Camera>, With<Authority>)>,
    mut player_query: Query<
        (
            &mut PreviousVelocity,
            &mut RigidBodyVelocity,
            &RigidBodyPosition,
            &ColliderShape,
            &RigidBodyColliders,
        ),
        With<Authority>,
    >,
) {
    let motion = input.movement_direction(camera_query.single().unwrap().rotation) * MOVE_SPEED;
    let (mut previous_velocity, mut velocity, position, shape, collider_handles) =
        player_query.single_mut().unwrap();

    velocity.linvel = previous_velocity.lerp(
        &motion.into(),
        VELOCITY_INTERPOLATE_SPEED * time.delta_seconds(),
    );

    if is_on_floor(
        &query_pipeline,
        &collider_query,
        &position.position,
        &**shape,
        &collider_handles.0,
    ) {
        if input.jumping {
            velocity.linvel.y += JUMP_IMPULSE;
        } else {
            velocity.linvel.y = 0.0;
        }
    } else {
        velocity.linvel.y -= GRAVITY * VELOCITY_INTERPOLATE_SPEED * time.delta_seconds();
    }
    previous_velocity.0 = velocity.linvel;
}

fn is_on_floor(
    query_pipeline: &Res<QueryPipeline>,
    collider_query: &QueryPipelineColliderComponentsQuery,
    position: &Isometry<Real>,
    shape: &dyn Shape,
    collider_handles: &[ColliderHandle],
) -> bool {
    query_pipeline
        .cast_shape(
            &QueryPipelineColliderComponentsSet(collider_query),
            position,
            &-Vector::x(),
            shape,
            FLOOR_THRESHOLD,
            InteractionGroups::all(),
            Some(&|handle| !collider_handles.contains(&handle)), // Exclude yourself
        )
        .is_some()
}

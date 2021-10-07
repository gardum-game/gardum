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

use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    ColliderHandle, ColliderShape, InteractionGroups, Isometry, QueryPipeline,
    QueryPipelineColliderComponentsQuery, QueryPipelineColliderComponentsSet, Real,
    RigidBodyColliders, RigidBodyPosition, RigidBodyVelocity, Shape, Vector,
};

use crate::app_state::AppState;

const MOVE_SPEED: f32 = 50.0;
const GRAVITY: f32 = 980.0;
const VELOCITY_INTERPOLATE_SPEED: f32 = 20.0;
const JUMP_IMPULSE: f32 = 200.0;
const FLOOR_THRESHOLD: f32 = 0.01;

pub struct PlayerController;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(input_system.system()),
        );
    }
}

fn input_system(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut query: Query<
        (
            &mut RigidBodyVelocity,
            &RigidBodyPosition,
            &ColliderShape,
            &RigidBodyColliders,
        ),
        With<PlayerController>,
    >,
) {
    let motion = movement_direction(&input) * MOVE_SPEED;
    let (mut velocity, position, shape, collider_handles) = query.single_mut().unwrap();

    velocity.linvel = velocity
        .linvel
        .lerp(&motion, VELOCITY_INTERPOLATE_SPEED * time.delta_seconds());

    if is_on_floor(
        &query_pipeline,
        &collider_query,
        &position.position,
        &**shape,
        &collider_handles.0,
    ) {
        if input.just_pressed(KeyCode::Space) {
            velocity.linvel.y += JUMP_IMPULSE;
        }
    } else {
        velocity.linvel.y -= GRAVITY * time.delta_seconds();
    }
}

fn movement_direction(input: &Res<Input<KeyCode>>) -> Vector<Real> {
    let mut motion = Vector::zeros();
    if input.pressed(KeyCode::W) {
        motion.x += 1.0;
    }
    if input.pressed(KeyCode::S) {
        motion.x -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        motion.z -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        motion.z += 1.0;
    }

    if motion != Vector::zeros() {
        motion.normalize_mut();
    }

    motion
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

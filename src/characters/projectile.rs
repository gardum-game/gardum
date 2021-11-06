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
use heron::{CollisionEvent, CollisionLayers, CollisionShape, RigidBody, Velocity};

use crate::core::{AppState, CollisionLayer};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ProjectileHitEvent>().add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(collision_system.system()),
        );
    }
}

fn collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut hit_events: EventWriter<ProjectileHitEvent>,
) {
    for (projectile, target) in collision_events.iter().filter_map(|event| {
        if event.is_started() {
            return None;
        }
        let (layers_1, layers_2) = event.collision_layers();
        if layers_1.contains_group(CollisionLayer::Projectile)
            && !layers_1.contains_group(CollisionLayer::Character)
        {
            return Some(event.rigid_body_entities());
        }
        if layers_2.contains_group(CollisionLayer::Projectile)
            && !layers_2.contains_group(CollisionLayer::Character)
        {
            let (target, projectile) = event.rigid_body_entities();
            return Some((projectile, target));
        }
        None
    }) {
        hit_events.send(ProjectileHitEvent { projectile, target });
        commands.entity(projectile).despawn();
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub rigid_body: RigidBody,
    pub shape: CollisionShape,
    pub collision_layers: CollisionLayers,
    pub velocity: Velocity,
    pub projectile: Projectile,

    #[bundle]
    pub pbr: PbrBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::new(
                CollisionLayer::Projectile,
                CollisionLayer::Character,
            ),
            velocity: Velocity::default(),
            projectile: Projectile,
            pbr: PbrBundle::default(),
        }
    }
}

pub struct Projectile;

#[allow(dead_code)]
struct ProjectileHitEvent {
    projectile: Entity,
    target: Entity,
}

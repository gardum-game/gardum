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
use heron::{CollisionEvent, CollisionShape, RigidBody, Velocity};

use crate::core::{AppState, CollisionLayer};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(collision_system.system()),
        );
    }
}

fn collision_system(
    query: Query<&Projectile>,
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
) {
    events
        .iter()
        .filter(|event| event.is_started())
        .filter_map(|event| {
            let (layers_1, layers_2) = event.collision_layers();
            if layers_1.contains_group(CollisionLayer::Projectile)
                && !layers_1.contains_group(CollisionLayer::World)
            {
                Some((
                    event.rigid_body_entities().0,
                    event.rigid_body_entities().1,
                    layers_2,
                ))
            } else if layers_2.contains_group(CollisionLayer::Projectile)
                && !layers_2.contains_group(CollisionLayer::World)
            {
                Some((
                    event.rigid_body_entities().1,
                    event.rigid_body_entities().0,
                    layers_1,
                ))
            } else {
                None
            }
        })
        .for_each(|(projectile_entity, other_entity, other_entity_layers)| {
            if other_entity_layers.contains_group(CollisionLayer::Player)
                && !other_entity_layers.contains_group(CollisionLayer::World)
            {
                if let Ok(projectile) = query.get(projectile_entity) {
                    (projectile.apply_on_hit)(&commands, other_entity);
                }
            }
            commands.entity(projectile_entity).despawn();
        });
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    rigid_body: RigidBody,
    shape: CollisionShape,
    velocity: Velocity,
    projectile: Projectile,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            velocity: Velocity::default(),
            projectile: Projectile::default(),
            pbr: PbrBundle::default(),
        }
    }
}

struct Projectile {
    apply_on_hit: fn(&Commands, Entity),
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            apply_on_hit: |_, _| (),
        }
    }
}

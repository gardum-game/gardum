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

pub(super) mod ability;
pub(super) mod character_action;
pub(super) mod cooldown;
mod despawn_timer;
pub(super) mod health;
pub(super) mod heroes;
mod movement;
mod orbit_camera;
mod projectile;

use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, Velocity};
use leafwing_input_manager::prelude::ActionState;

use crate::core::CollisionLayer;
use ability::Abilities;
use ability::AbilityPlugin;
use character_action::CharacterAction;
use character_action::CharacterActionPlugin;
use cooldown::CooldownPlugin;
use despawn_timer::DespawnTimerPlugin;
use health::Health;
use health::HealthPlugin;
use heroes::HeroesPlugin;
use movement::MovementPlugin;
use orbit_camera::OrbitCameraPlugin;
use projectile::ProjectilePlugin;

pub(super) struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CharacterActionPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(CooldownPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(DespawnTimerPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(HeroesPlugin)
            .add_plugin(ProjectilePlugin);
    }
}

#[derive(Bundle)]
pub(super) struct CharacterBundle {
    health: Health,
    abilities: Abilities,
    rigid_body: RigidBody,
    shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,
    action_state: ActionState<CharacterAction>,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            health: Health::default(),
            abilities: Abilities::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::all::<CollisionLayer>()
                .with_group(CollisionLayer::Character),
            velocity: Velocity::default(),
            action_state: ActionState::default(),
            pbr: PbrBundle::default(),
        }
    }
}

/// Used to store reference to the owner
#[derive(Component)]
pub(super) struct Owner(pub(crate) Entity);

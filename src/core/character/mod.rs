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

pub(crate) mod hero;

use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody, Velocity};
use leafwing_input_manager::prelude::ActionState;

use super::{
    ability::Abilities, character_action::CharacterAction, health::Health, CollisionLayer,
};
use hero::HeroesPlugin;

pub(super) struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HeroesPlugin);
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
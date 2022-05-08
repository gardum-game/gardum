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
use derive_more::{AddAssign, From, SubAssign};
use heron::{CollisionLayers, CollisionShape, RigidBody, RotationConstraints, Velocity};
use leafwing_input_manager::prelude::ActionState;

use super::{ability::Abilities, control_actions::ControlAction, health::Health, CollisionLayer};
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
    speed_modifier: SpeedModifier,
    damage_modifier: DamageModifier,
    healing_modifier: HealingModifier,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
    shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,
    action_state: ActionState<ControlAction>,

    #[bundle]
    pbr: PbrBundle,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            health: Health::default(),
            abilities: Abilities::default(),
            speed_modifier: SpeedModifier::default(),
            damage_modifier: DamageModifier::default(),
            healing_modifier: HealingModifier::default(),
            rigid_body: RigidBody::Dynamic,
            rotation_constraints: RotationConstraints::lock(),
            shape: CollisionShape::default(),
            collision_layers: CollisionLayers::all_masks::<CollisionLayer>()
                .with_group(CollisionLayer::Character),
            velocity: Velocity::default(),
            action_state: ActionState::default(),
            pbr: PbrBundle::default(),
        }
    }
}

/// Movement speed modifier
#[derive(Component, AddAssign, SubAssign, Clone, Copy, From)]
pub(super) struct SpeedModifier(pub(super) f32);

impl Default for SpeedModifier {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Outgoing damage modifier
#[derive(Component, AddAssign, SubAssign, Clone, Copy, From)]
pub(super) struct DamageModifier(pub(super) f32);

impl Default for DamageModifier {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Outgoing healing modifier
#[derive(Component, AddAssign, SubAssign, Clone, Copy, From)]
pub(super) struct HealingModifier(pub(super) f32);

impl Default for HealingModifier {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Returns normalized direction (without Y coordinate).
/// Returns `-Vec3::Z` if the camera rotation is facing down
fn character_direction(camera_rotation: Quat) -> Vec3 {
    let mut direction = camera_rotation * -Vec3::Z;
    direction.y = 0.0;
    direction.try_normalize().unwrap_or(-Vec3::Z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_direction_from_camera() {
        for (rotation, expected_direction) in [
            (Quat::from_rotation_x(90_f32.to_radians()), -Vec3::Z),
            (Quat::from_rotation_y(90_f32.to_radians()), -Vec3::X),
            (Quat::from_rotation_z(90_f32.to_radians()), -Vec3::Z),
            (Quat::from_rotation_x(-90_f32.to_radians()), -Vec3::Z),
            (Quat::from_rotation_y(-90_f32.to_radians()), Vec3::X),
            (Quat::from_rotation_z(-90_f32.to_radians()), -Vec3::Z),
        ] {
            assert_eq!(
                character_direction(rotation),
                expected_direction,
                "Character direction from {rotation} should be equal to {expected_direction}"
            );
        }
    }
}

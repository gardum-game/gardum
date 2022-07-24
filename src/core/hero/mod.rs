/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

mod north;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use derive_more::{AddAssign, From, SubAssign};
use leafwing_input_manager::prelude::*;
use strum::{EnumIter, EnumString};

use super::{ability::Abilities, control_actions::ControlAction, health::Health, CollisionMask};
use north::NorthPlugin;

pub(super) struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NorthPlugin);
    }
}

#[derive(Bundle)]
pub(crate) struct HeroBundle {
    hero_kind: HeroKind,
    health: Health,
    speed_modifier: SpeedModifier,
    damage_modifier: DamageModifier,
    healing_modifier: HealingModifier,
    transform: Transform,
    action_state: ActionState<ControlAction>,
}

impl HeroBundle {
    pub(crate) fn new(hero_kind: HeroKind, translation: Vec3) -> Self {
        Self {
            hero_kind,
            health: Health::default(),
            speed_modifier: SpeedModifier::default(),
            damage_modifier: DamageModifier::default(),
            healing_modifier: HealingModifier::default(),
            transform: Transform::from_translation(translation),
            action_state: ActionState::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, EnumIter, EnumString, Debug, Component)]
pub(crate) enum HeroKind {
    North,
}

#[derive(Bundle)]
pub(super) struct LocalHeroBundle {
    abilities: Abilities,
    rigid_body: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

impl Default for LocalHeroBundle {
    fn default() -> Self {
        Self {
            abilities: Abilities::default(),
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(0.5, 0.5),
            collision_groups: CollisionGroups {
                memberships: CollisionMask::CHARACTER.bits(),
                filters: CollisionMask::all().bits(),
            },
            mesh: Default::default(),
            material: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
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

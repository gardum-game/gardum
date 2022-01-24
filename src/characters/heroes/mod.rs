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

mod north;

use bevy::prelude::*;
use strum::EnumIter;

use super::{ability::Abilities, CharacterBundle};
use north::NorthPlugin;

pub(super) struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NorthPlugin);
    }
}

#[derive(Bundle)]
pub struct HeroBundle {
    abilities: Abilities,

    #[bundle]
    character: CharacterBundle,
}

impl HeroBundle {
    /// Create hero bundle from the specified kind
    pub(crate) fn hero(
        kind: HeroKind,
        transform: Transform,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let create_fn = match kind {
            HeroKind::North => HeroBundle::north,
        };
        create_fn(transform, commands, meshes, materials)
    }
}

#[derive(Clone, Copy, PartialEq, EnumIter, Debug, Component)]
pub(crate) enum HeroKind {
    North,
}

/// Used to store reference to the hero
#[derive(Component)]
pub(super) struct OwnerHero(pub(crate) Entity);

#[cfg(test)]
mod tests {
    use bevy::ecs::system::SystemState;
    use strum::IntoEnumIterator;

    use super::*;
    use crate::{
        characters::{
            ability::ActivationEvent,
            health::{DamageEvent, HealEvent},
            projectile::ProjectileHitEvent,
        },
        test_utils::HeadlessRenderPlugin,
    };

    #[test]
    fn hero_bundle() {
        let mut app = setup_app();
        let mut system_state: SystemState<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
        )> = SystemState::new(&mut app.world);
        let (mut commands, mut meshes, mut materials) = system_state.get_mut(&mut app.world);

        for kind in HeroKind::iter() {
            HeroBundle::hero(
                kind,
                Transform::default(),
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_event::<ActivationEvent>()
            .add_event::<ProjectileHitEvent>()
            .add_event::<DamageEvent>()
            .add_event::<HealEvent>()
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(HeroesPlugin);

        app
    }
}

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
use crate::core::{AppState, Authority};
use north::NorthPlugin;

pub(super) struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NorthPlugin).add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(hero_authority_system),
        );
    }
}

/// Give authority to the hero if it's player have authority
fn hero_authority_system(
    mut commands: Commands,
    hero_query: Query<(Entity, &OwnerPlayer), Added<OwnerPlayer>>,
    authority_query: Query<(), With<Authority>>,
) {
    for (hero, player) in hero_query.iter() {
        if authority_query.get(player.0).is_ok() {
            commands.entity(hero).insert(Authority);
        }
    }
}

#[derive(Bundle)]
pub struct HeroBundle {
    player: OwnerPlayer,
    kind: HeroKind,
    abilities: Abilities,

    #[bundle]
    character: CharacterBundle,
}

impl HeroBundle {
    /// Create hero bundle from the specified kind
    pub(crate) fn hero(
        kind: HeroKind,
        player: OwnerPlayer,
        transform: Transform,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let create_fn = match kind {
            HeroKind::North => HeroBundle::north,
        };
        create_fn(player, transform, commands, meshes, materials)
    }
}

#[derive(Clone, Copy, PartialEq, EnumIter, Debug, Component)]
pub(crate) enum HeroKind {
    North,
}

/// Used to store hero's player entity
#[derive(Component)]
pub(crate) struct OwnerPlayer(pub(crate) Entity);

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
    fn hero_inherits_authority() {
        let mut app = setup_app();
        let player = app.world.spawn().id();
        let hero = app.world.spawn().insert(OwnerPlayer(player)).id();

        app.update();

        assert!(
            app.world.get::<Authority>(hero).is_none(),
            "Hero shouldn't have authority"
        );

        let player = app.world.entity_mut(player).insert(Authority).id();
        let hero = app.world.spawn().insert(OwnerPlayer(player)).id();

        app.update();

        assert!(
            app.world.get::<Authority>(hero).is_some(),
            "Hero should have authority"
        );
    }

    #[test]
    fn hero_bundle() {
        let mut app = setup_app();
        let player = app.world.spawn().id();
        let mut system_state: SystemState<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<StandardMaterial>>,
        )> = SystemState::new(&mut app.world);
        let (mut commands, mut meshes, mut materials) = system_state.get_mut(&mut app.world);

        for kind in HeroKind::iter() {
            let hero_bundle = HeroBundle::hero(
                kind,
                OwnerPlayer(player),
                Transform::default(),
                &mut commands,
                &mut meshes,
                &mut materials,
            );
            assert_eq!(
                hero_bundle.kind, kind,
                "Hero kind in bundle should be equal to specified"
            )
        }
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_event::<ActivationEvent>()
            .add_event::<ProjectileHitEvent>()
            .add_event::<DamageEvent>()
            .add_event::<HealEvent>()
            .add_state(AppState::InGame)
            .add_plugin(HeadlessRenderPlugin)
            .add_plugin(HeroesPlugin);

        app
    }
}

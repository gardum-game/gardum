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

pub mod north;

use bevy::prelude::*;
use strum::EnumIter;

use super::{ability::Abilities, CharacterBundle};
use crate::core::{player::PlayerHero, AppState, Authority};
use north::NorthPlugin;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HeroSelectEvent>()
            .add_plugin(NorthPlugin)
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame).with_system(hero_selection_system),
            );
    }
}

fn hero_selection_system(
    mut commands: Commands,
    mut selection_events: EventReader<HeroSelectEvent>,
    player_query: Query<(Option<&PlayerHero>, Option<&Authority>)>,
    #[cfg(feature = "client")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "client")] mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in selection_events.iter() {
        let hero_bundle = HeroBundle::hero(
            event.kind,
            OwnerPlayer(event.player),
            event.transform,
            &mut commands,
            #[cfg(feature = "client")]
            &mut meshes,
            #[cfg(feature = "client")]
            &mut materials,
        );

        let (hero, authority) = player_query.get(event.player).unwrap();
        if let Some(hero) = hero {
            commands.entity(hero.0).despawn();
        }

        let mut entity_commands = commands.spawn_bundle(hero_bundle);
        if authority.is_some() {
            entity_commands.insert(Authority);
        }
    }
}

#[derive(Bundle)]
struct HeroBundle {
    player: OwnerPlayer,
    kind: HeroKind,
    abilities: Abilities,

    #[bundle]
    character: CharacterBundle,
}

impl HeroBundle {
    /// Create hero bundle from the specified kind
    fn hero(
        kind: HeroKind,
        player: OwnerPlayer,
        transform: Transform,
        commands: &mut Commands,
        #[cfg(feature = "client")] meshes: &mut Assets<Mesh>,
        #[cfg(feature = "client")] materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let create_fn = match kind {
            HeroKind::North => HeroBundle::north,
        };
        create_fn(
            player,
            transform,
            commands,
            #[cfg(feature = "client")]
            meshes,
            #[cfg(feature = "client")]
            materials,
        )
    }
}

#[derive(Clone, Copy, PartialEq, EnumIter, Debug, Component)]
pub enum HeroKind {
    North,
}

/// Used to store hero's player entity
#[derive(Component)]
pub struct OwnerPlayer(pub Entity);

pub struct HeroSelectEvent {
    pub player: Entity,
    pub kind: HeroKind,
    pub transform: Transform,
}

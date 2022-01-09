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
use crate::core::{AppState, Authority};
use north::NorthPlugin;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<HeroSelectEvent>()
            .add_plugin(NorthPlugin)
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame)
                    .with_system(hero_selection_system.system()),
            );
    }
}

fn hero_selection_system(
    mut commands: Commands,
    mut spawn_events: EventReader<HeroSelectEvent>,
    authority_query: Query<(), With<Authority>>,
    #[cfg(feature = "client")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "client")] mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in spawn_events.iter() {
        let hero_bundle = match event.kind {
            HeroKind::North => HeroBundle::north(
                PlayerOwner(event.player),
                event.transform,
                &mut commands,
                #[cfg(feature = "client")]
                &mut meshes,
                #[cfg(feature = "client")]
                &mut materials,
            ),
        };

        let mut entity_commands = commands.spawn_bundle(hero_bundle);
        if authority_query.get(event.player).is_ok() {
            entity_commands.insert(Authority);
        }
    }
}

#[derive(Bundle)]
struct HeroBundle {
    player: PlayerOwner,
    kind: HeroKind,
    abilities: Abilities,

    #[bundle]
    character: CharacterBundle,
}

#[derive(Clone, Copy, PartialEq, EnumIter, Debug)]
pub enum HeroKind {
    North,
}

/// Used to store hero's player entity
pub struct PlayerOwner(pub Entity);

pub struct HeroSelectEvent {
    pub player: Entity,
    pub kind: HeroKind,
    pub transform: Transform,
}

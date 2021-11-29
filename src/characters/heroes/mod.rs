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

pub mod north;

use bevy::prelude::*;
use strum::EnumIter;

use super::{ability::Abilities, CharacterBundle};
use crate::core::{AppState, Authority};
use north::NorthPlugin;

pub struct HeroesPlugin;

impl Plugin for HeroesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<HeroSpawnEvent>()
            .add_plugin(NorthPlugin)
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(spawn_hero_system.system()),
            );
    }
}

fn spawn_hero_system(
    mut commands: Commands,
    mut spawn_events: EventReader<HeroSpawnEvent>,
    authority_query: Query<(), With<Authority>>,
    #[cfg(not(feature = "headless"))] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(not(feature = "headless"))] mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in spawn_events.iter() {
        let hero_bundle = match event.kind {
            HeroKind::North => HeroBundle::north(
                &mut commands,
                event.transform,
                #[cfg(not(feature = "headless"))]
                &mut meshes,
                #[cfg(not(feature = "headless"))]
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
    abilities: Abilities,
    kind: HeroKind,

    #[bundle]
    character: CharacterBundle,
}

#[derive(Clone, Copy, PartialEq, EnumIter, Debug)]
pub enum HeroKind {
    North,
}

pub struct HeroSpawnEvent {
    pub player: Entity,
    pub kind: HeroKind,
    pub transform: Transform,
}

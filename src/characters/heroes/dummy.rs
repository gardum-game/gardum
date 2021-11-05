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
use heron::CollisionShape;

use super::{Hero, HeroBundle, HeroSpawnEvent};
use crate::{
    characters::{
        abilities::{Abilities, AbilitySlot, ActivationEvent, Cooldown},
        CharacterBundle,
    },
    core::{AppState, Authority},
};

pub struct DummyPlugin;

impl Plugin for DummyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_dummy_system.system())
                .with_system(frost_bolt_system.system()),
        );
    }
}

fn spawn_dummy_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_events: EventReader<HeroSpawnEvent>,
) {
    for event in spawn_events
        .iter()
        .filter(|event| event.hero == Hero::Dummy)
    {
        let abilities = Abilities(vec![commands.spawn_bundle(FrostBoltBundle::default()).id()]);
        let mut entity_commands = commands.spawn_bundle(HeroBundle {
            abilities,
            hero: event.hero,
            character: CharacterBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                    material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                    transform: event.transform,
                    ..Default::default()
                },
                shape: CollisionShape::Capsule {
                    half_segment: 0.5,
                    radius: 0.5,
                },
                ..Default::default()
            },
        });
        if event.authority {
            entity_commands.insert(Authority);
        }
    }
}

fn frost_bolt_system(mut events: EventReader<ActivationEvent>, query: Query<&FrostBoltAbility>) {
    for _ in events
        .iter()
        .filter(|event| query.get(event.ability).is_ok())
    {
        println!("Called!");
    }
}

#[derive(Bundle)]
struct FrostBoltBundle {
    kind: FrostBoltAbility,
    slot: AbilitySlot,
    cooldown: Cooldown,
}

impl Default for FrostBoltBundle {
    fn default() -> Self {
        Self {
            kind: FrostBoltAbility,
            slot: AbilitySlot::BaseAttack,
            cooldown: Cooldown::from_secs(4),
        }
    }
}

struct FrostBoltAbility;

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

use crate::core::{AppState, Damage, Deaths, Healing, Kills, Player};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<DamageEvent>()
            .add_event::<HealEvent>()
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame)
                    .with_system(heal_system.system())
                    .with_system(damage_system.system()),
            );
    }
}

fn heal_system(
    mut events: EventReader<HealEvent>,
    mut target_query: Query<&mut Health>,
    instigator_query: Query<&Player>,
    mut instigator_player_query: Query<&mut Healing>,
) {
    for event in events.iter() {
        let mut health = target_query.get_mut(event.target).unwrap();
        if health.current == 0 {
            continue;
        }

        let delta = event.heal.min(health.max - health.current);
        health.current += delta;

        let player = instigator_query.get(event.instigator).unwrap();
        let mut healing = instigator_player_query.get_mut(player.0).unwrap();
        healing.0 += delta;
    }
}

fn damage_system(
    mut events: EventReader<DamageEvent>,
    mut target_query: Query<(&Player, &mut Health)>,
    mut target_player_query: Query<&mut Deaths>,
    instigator_query: Query<&Player>,
    mut instigator_player_query: Query<(&mut Damage, &mut Kills)>,
) {
    for event in events.iter() {
        let (target_player, mut health) = target_query.get_mut(event.target).unwrap();
        let delta = health.current.min(event.damage);
        health.current -= delta;
        if health.current == 0 {
            let mut deaths = target_player_query.get_mut(target_player.0).unwrap();
            deaths.0 += 1;
        }

        if event.target != event.instigator {
            let instigator_player = instigator_query.get(event.instigator).unwrap();
            let (mut damage, mut kills) = instigator_player_query
                .get_mut(instigator_player.0)
                .unwrap();
            damage.0 += delta;

            if health.current == 0 {
                kills.0 += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Health {
    pub current: usize,
    pub max: usize,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
        }
    }
}

pub struct HealEvent {
    pub instigator: Entity,
    pub target: Entity,
    pub heal: usize,
}

pub struct DamageEvent {
    pub instigator: Entity,
    pub target: Entity,
    pub damage: usize,
}

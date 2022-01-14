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

use bevy::prelude::*;

use super::heroes::OwnerPlayer;
use crate::core::{
    player::{Damage, Deaths, Healing, Kills},
    AppState,
};

pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<HealEvent>()
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::InGame)
                    .with_system(heal_system)
                    .with_system(damage_system),
            );
    }
}

fn heal_system(
    mut events: EventReader<HealEvent>,
    mut target_query: Query<&mut Health>,
    instigator_query: Query<&OwnerPlayer>,
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
    mut target_query: Query<(&OwnerPlayer, &mut Health)>,
    mut target_player_query: Query<&mut Deaths>,
    instigator_query: Query<&OwnerPlayer>,
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

#[derive(Component, Debug, PartialEq)]
pub(crate) struct Health {
    pub(crate) current: u32,
    pub(crate) max: u32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
        }
    }
}

pub(super) struct HealEvent {
    pub(super) instigator: Entity,
    pub(super) target: Entity,
    pub(super) heal: u32,
}

pub(super) struct DamageEvent {
    pub(super) instigator: Entity,
    pub(super) target: Entity,
    pub(super) damage: u32,
}

#[cfg(test)]
mod tests {
    use bevy::app::Events;

    use super::*;
    use crate::core::player::PlayerBundle;

    #[test]
    fn healing() {
        let mut app = setup_app();
        let target_player = app
            .world
            .spawn()
            .insert_bundle(PlayerBundle::default())
            .id();
        let target = app
            .world
            .spawn()
            .insert(Health::default())
            .insert(OwnerPlayer(target_player))
            .id();

        let instigator_player = app
            .world
            .spawn()
            .insert_bundle(PlayerBundle::default())
            .id();
        let instigator = app
            .world
            .spawn()
            .insert(OwnerPlayer(instigator_player))
            .id();

        for (initial_health, heal, expected_healing, expected_health) in [
            (90, 5, 5, 95),
            (90, 20, 10, Health::default().max),
            (90, 10, 10, Health::default().max),
            (0, 20, 0, 0),
        ] {
            app.world.get_mut::<Health>(target).unwrap().current = initial_health;
            app.world.get_mut::<Healing>(instigator_player).unwrap().0 = 0;

            let mut events = app.world.get_resource_mut::<Events<HealEvent>>().unwrap();
            events.send(HealEvent {
                instigator,
                target,
                heal,
            });

            app.update();

            let health = app.world.get::<Health>(target).unwrap();
            assert_eq!(
                health.current, expected_health,
                "Healing from {} for {} points should set health to {}",
                initial_health, heal, expected_health
            );

            let healing = app.world.get::<Healing>(instigator_player).unwrap();
            assert_eq!(
                healing.0, expected_healing,
                "Healing from {} for {} points should set amount of healing to {}",
                initial_health, heal, expected_healing
            );
        }
    }

    #[test]
    fn damaging() {
        let mut app = setup_app();
        let target_player = app
            .world
            .spawn()
            .insert_bundle(PlayerBundle::default())
            .id();
        let target = app
            .world
            .spawn()
            .insert(Health::default())
            .insert(OwnerPlayer(target_player))
            .id();

        let instigator_player = app
            .world
            .spawn()
            .insert_bundle(PlayerBundle::default())
            .id();
        let instigator = app
            .world
            .spawn()
            .insert(OwnerPlayer(instigator_player))
            .id();

        for (initial_health, damage, expected_damage, expected_health) in [
            (90, 5, 5, 85),
            (90, 95, 90, 0),
            (90, 90, 90, 0),
            (0, 20, 0, 0),
        ] {
            app.world.get_mut::<Health>(target).unwrap().current = initial_health;
            app.world.get_mut::<Damage>(instigator_player).unwrap().0 = 0;

            let mut events = app.world.get_resource_mut::<Events<DamageEvent>>().unwrap();
            events.send(DamageEvent {
                instigator,
                target,
                damage,
            });

            app.update();

            let health = app.world.get::<Health>(target).unwrap();
            assert_eq!(
                health.current, expected_health,
                "Damaging from {} for {} points should set health to {}",
                initial_health, damage, expected_health
            );

            let damaging = app.world.get::<Damage>(instigator_player).unwrap();
            assert_eq!(
                damaging.0, expected_damage,
                "Damaging from {} for {} points should set amount of damage to {}",
                initial_health, damage, expected_damage
            );

            if health.current == 0 {
                let kills = app.world.get::<Kills>(instigator_player).unwrap();
                assert_eq!(
                    kills.0, 1,
                    "The instigator gets a kill if the target's health drops to 0"
                );

                let deaths = app.world.get::<Deaths>(target_player).unwrap();
                assert_eq!(
                    deaths.0, 1,
                    "The target gets a death if its health drops to 0"
                );

                // Reset for the next iteration
                app.world.get_mut::<Kills>(instigator_player).unwrap().0 = 0;
                app.world.get_mut::<Deaths>(target_player).unwrap().0 = 0;
            }
        }
    }

    #[test]
    fn self_damaging() {
        let damage = Health::default().max;

        let mut app = setup_app();
        let target_player = app
            .world
            .spawn()
            .insert_bundle(PlayerBundle::default())
            .id();
        let target = app
            .world
            .spawn()
            .insert(Health::default())
            .insert(OwnerPlayer(target_player))
            .id();

        let mut events = app.world.get_resource_mut::<Events<DamageEvent>>().unwrap();
        events.send(DamageEvent {
            instigator: target,
            target,
            damage,
        });

        app.update();

        let health = app.world.get::<Health>(target).unwrap();
        assert_eq!(
            health.current,
            Health::default().current - damage,
            "Health should decrease by the amount of damage"
        );

        let healing = app.world.get::<Damage>(target_player).unwrap();
        assert_eq!(
            healing.0, 0,
            "Amount of damage shouldn't increase for self-damage"
        );

        let kills = app.world.get::<Kills>(target_player).unwrap();
        assert_eq!(kills.0, 0, "Kills shouldn't counted for self-damage");

        let deaths = app.world.get::<Deaths>(target_player).unwrap();
        assert_eq!(deaths.0, 1, "Deaths should counted for self-damage");
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(HealthPlugin);

        app
    }
}

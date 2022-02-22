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

use super::{
    character::{DamageModifier, HealingModifier},
    player::{Damage, Deaths, Healing, Kills},
    AppState,
};

pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthChangeEvent>().add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(heal_system)
                .with_system(damage_system),
        );
    }
}

fn heal_system(
    mut health_events: EventReader<HealthChangeEvent>,
    mut targets: Query<&mut Health>,
    mut instigators: Query<(&mut Healing, &HealingModifier)>,
) {
    for event in health_events.iter().filter(|event| event.delta > 0) {
        let mut health = targets.get_mut(event.target).unwrap();
        if health.current == 0 {
            continue;
        }

        let (mut healing, healing_modifier) = instigators.get_mut(event.instigator).unwrap();
        let delta = health
            .missing()
            .min((event.delta as f32 * healing_modifier.0) as u32);
        health.current += delta;
        healing.0 += delta;
    }
}

fn damage_system(
    mut health_events: EventReader<HealthChangeEvent>,
    mut targets: Query<(&mut Health, &mut Deaths)>,
    mut instigators: Query<(&mut Damage, &mut Kills, &DamageModifier)>,
    mut commands: Commands,
) {
    for event in health_events.iter().filter(|event| event.delta < 0) {
        let (mut health, mut deaths) = targets.get_mut(event.target).unwrap();
        let (mut damage, mut kills, damage_modifier) =
            instigators.get_mut(event.instigator).unwrap();

        let delta = health
            .current
            .min((event.delta.abs() as f32 * damage_modifier.0) as u32);
        health.current -= delta;
        if health.current == 0 {
            deaths.0 += 1;
            commands.entity(event.target).insert(Death);
        }

        if event.target != event.instigator {
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

impl Health {
    fn missing(&self) -> u32 {
        self.max - self.current
    }
}

pub(super) struct HealthChangeEvent {
    pub(super) instigator: Entity,
    pub(super) target: Entity,
    pub(super) delta: i32,
}

#[derive(Component)]
pub(super) struct Death;

#[cfg(test)]
mod tests {
    use bevy::app::Events;

    use super::*;
    use crate::core::player::PlayerBundle;

    #[test]
    fn healing() {
        let mut app = setup_app();
        let target = app
            .world
            .spawn()
            .insert(Health::default())
            .insert_bundle(PlayerBundle::default())
            .id();
        let instigator = app
            .world
            .spawn()
            .insert(HealingModifier::default())
            .insert_bundle(PlayerBundle::default())
            .id();

        for (initial_health, delta, expected_healing, expected_health, modifier) in [
            (90, 5, 5, 95, 1.0),
            (90, 20, 10, Health::default().max, 1.0),
            (90, 10, 10, Health::default().max, 1.0),
            (0, 20, 0, 0, 1.0),
            (85, 5, 10, 95, 2.0),
        ] {
            app.world.get_mut::<Health>(target).unwrap().current = initial_health;
            app.world.get_mut::<HealingModifier>(instigator).unwrap().0 = modifier;
            app.world.get_mut::<Healing>(instigator).unwrap().0 = 0;

            let mut health_events = app
                .world
                .get_resource_mut::<Events<HealthChangeEvent>>()
                .unwrap();
            health_events.send(HealthChangeEvent {
                instigator,
                target,
                delta,
            });

            app.update();

            let health = app.world.get::<Health>(target).unwrap();
            assert_eq!(
                health.current, expected_health,
                "Healing from {initial_health} for {delta} points should set health to {expected_health}",
            );

            let healing = app.world.get::<Healing>(instigator).unwrap();
            assert_eq!(
                healing.0, expected_healing,
                "Healing from {initial_health} for {delta} points should set amount of healing to {expected_healing}",
            );
        }
    }

    #[test]
    fn damaging() {
        let mut app = setup_app();
        let target = app
            .world
            .spawn()
            .insert(Health::default())
            .insert_bundle(PlayerBundle::default())
            .id();
        let instigator = app
            .world
            .spawn()
            .insert_bundle(PlayerBundle::default())
            .insert(DamageModifier::default())
            .id();

        for (initial_health, delta, expected_damage, expected_health, modifier) in [
            (90, -5, 5, 85, 1.0),
            (90, -95, 90, 0, 1.0),
            (90, -90, 90, 0, 1.0),
            (0, -20, 0, 0, 1.0),
            (90, -5, 10, 80, 2.0),
        ] {
            app.world.get_mut::<Health>(target).unwrap().current = initial_health;
            app.world.get_mut::<Damage>(instigator).unwrap().0 = 0;
            app.world.get_mut::<DamageModifier>(instigator).unwrap().0 = modifier;

            let mut health_events = app
                .world
                .get_resource_mut::<Events<HealthChangeEvent>>()
                .unwrap();
            health_events.send(HealthChangeEvent {
                instigator,
                target,
                delta,
            });

            app.update();

            let health = app.world.get::<Health>(target).unwrap();
            assert_eq!(
                health.current, expected_health,
                "Damaging from {initial_health} for {delta} points should set health to {expected_health}",
            );

            let damaging = app.world.get::<Damage>(instigator).unwrap();
            assert_eq!(
                damaging.0, expected_damage,
                "Damaging from {initial_health} for {delta} points should set amount of damage to {expected_damage}",
            );

            if health.current == 0 {
                let kills = app.world.get::<Kills>(instigator).unwrap();
                assert_eq!(
                    kills.0, 1,
                    "The instigator gets a kill if the target's health drops to 0"
                );

                let deaths = app.world.get::<Deaths>(target).unwrap();
                assert_eq!(
                    deaths.0, 1,
                    "The target gets a death if its health drops to 0"
                );

                app.world
                    .get::<Death>(target)
                    .expect("Target should have a Death component");

                // Reset for the next iteration
                app.world.get_mut::<Kills>(instigator).unwrap().0 = 0;
                app.world.get_mut::<Deaths>(target).unwrap().0 = 0;
            }
        }
    }

    #[test]
    fn self_damaging() {
        let mut app = setup_app();
        let target = app
            .world
            .spawn()
            .insert(Health::default())
            .insert(DamageModifier::default())
            .insert_bundle(PlayerBundle::default())
            .id();

        let delta = -(Health::default().max as i32);
        let mut health_events = app
            .world
            .get_resource_mut::<Events<HealthChangeEvent>>()
            .unwrap();
        health_events.send(HealthChangeEvent {
            instigator: target,
            target,
            delta,
        });

        app.update();

        let health = app.world.get::<Health>(target).unwrap();
        assert_eq!(
            health.current,
            health.max - delta.abs() as u32,
            "Health should decrease by the amount of damage"
        );

        let healing = app.world.get::<Damage>(target).unwrap();
        assert_eq!(
            healing.0, 0,
            "Amount of damage shouldn't increase for self-damage"
        );

        let kills = app.world.get::<Kills>(target).unwrap();
        assert_eq!(kills.0, 0, "Kills shouldn't counted for self-damage");

        let deaths = app.world.get::<Deaths>(target).unwrap();
        assert_eq!(deaths.0, 1, "Deaths should counted for self-damage");

        app.world
            .get::<Death>(target)
            .expect("Target should have a Death component");
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugins(MinimalPlugins)
            .add_plugin(HealthPlugin);

        app
    }
}

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
use derive_more::{Deref, DerefMut};
use std::time::Duration;

use crate::core::{AppState, Authority};

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Option<AbilitySlot>>()
            .add_event::<ActivationEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(input_system.system())
                    .with_system(cooldown_system.system())
                    .with_system(activation_system.system())
                    .with_system(abilities_children_system.system()),
            );
    }
}

fn input_system(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut input: ResMut<Option<AbilitySlot>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        *input = Some(AbilitySlot::Ability1);
        return;
    }

    if keys.just_pressed(KeyCode::E) {
        *input = Some(AbilitySlot::Ability2);
        return;
    }

    if keys.just_pressed(KeyCode::LShift) {
        *input = Some(AbilitySlot::Ability3);
        return;
    }

    if keys.just_pressed(KeyCode::R) {
        *input = Some(AbilitySlot::Ultimate);
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        *input = Some(AbilitySlot::BaseAttack);
        return;
    }

    *input = None;
}

fn cooldown_system(time: Res<Time>, mut query: Query<&mut Cooldown>) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick(time.delta());
    }
}

fn activation_system(
    activated_slot: Res<Option<AbilitySlot>>,
    mut events: EventWriter<ActivationEvent>,
    caster_query: Query<(Entity, &Abilities), With<Authority>>,
    mut abilities_query: Query<(Entity, &AbilitySlot, Option<&mut Cooldown>)>,
) {
    let input = match *activated_slot {
        Some(input) => input,
        None => return,
    };

    for (caster, abilities) in caster_query.iter() {
        for child in abilities.iter() {
            let (ability, slot, cooldown) = abilities_query.get_mut(*child).unwrap();

            if input != *slot {
                continue;
            }

            if let Some(mut cooldown) = cooldown {
                if !cooldown.finished() {
                    return;
                }
                cooldown.reset();
            }

            events.send(ActivationEvent { caster, ability });
            return;
        }
    }
}

fn abilities_children_system(
    mut commands: Commands,
    query: Query<(Entity, &Abilities), Added<Abilities>>,
) {
    if let Ok((entity, abilities)) = query.single() {
        commands.entity(entity).push_children(abilities);
    }
}

pub struct ActivationEvent {
    pub caster: Entity,
    pub ability: Entity,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AbilitySlot {
    BaseAttack,
    Ability1,
    Ability2,
    Ability3,
    Ultimate,
}

#[derive(Deref, DerefMut)]
pub struct Abilities(pub Vec<Entity>);

#[derive(Deref, DerefMut)]
pub struct Cooldown(Timer);

impl Cooldown {
    pub fn from_secs(secs: u64) -> Self {
        // Setup timer in finished state
        let duration = Duration::from_secs(secs);
        let mut timer = Timer::new(duration, false);
        timer.tick(duration);

        Self(timer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_from_secs() {
        const SECONDS: u64 = 4;

        let cooldown = Cooldown::from_secs(SECONDS);
        assert_eq!(cooldown.duration(), Duration::from_secs(SECONDS));
        assert!(
            cooldown.finished(),
            "Object should be in finished state after creation"
        );
    }
}

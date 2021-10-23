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
use derive_more::{Deref, DerefMut, From, IntoIterator};
use std::time::Duration;

use crate::core::{AppState, Authority};

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AbilityInput::None).add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(input_system.system())
                .with_system(cooldown_system.system())
                .with_system(cast_system.system()),
        );
    }
}

fn input_system(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut input: ResMut<AbilityInput>,
) {
    if keys.just_pressed(KeyCode::Q) {
        *input = AbilityInput::Ability1;
        return;
    }

    if keys.just_pressed(KeyCode::E) {
        *input = AbilityInput::Ability2;
        return;
    }

    if keys.just_pressed(KeyCode::LShift) {
        *input = AbilityInput::Ability3;
        return;
    }

    if keys.just_pressed(KeyCode::R) {
        *input = AbilityInput::Ultimate;
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        *input = AbilityInput::BaseAttack;
        return;
    }

    *input = AbilityInput::None;
}

fn cooldown_system(time: Res<Time>, mut query: Query<&mut Abilities>) {
    for mut abilities in query.iter_mut() {
        for ability in abilities.iter_mut() {
            ability.cooldown.tick(time.delta());
        }
    }
}

fn cast_system(input: Res<AbilityInput>, mut query: Query<&mut Abilities, With<Authority>>) {
    if *input == AbilityInput::None {
        return;
    }

    let mut abilities = query.single_mut().unwrap();
    if let Some(ability) = abilities.get_mut(*input as usize) {
        ability.cast();
    }
}

#[derive(Copy, Clone, PartialEq)]
enum AbilityInput {
    None = -1,
    BaseAttack,
    Ability1,
    Ability2,
    Ability3,
    Ultimate,
}

#[derive(Default, Deref, DerefMut, IntoIterator, From)]
pub struct Abilities(Vec<Ability>);

pub struct Ability {
    pub logic: fn(),
    pub cooldown: Timer,
}

impl Ability {
    pub fn new(logic: fn(), secs: u64) -> Self {
        // Setup timer in finished state
        let duration = Duration::from_secs(secs);
        let mut cooldown = Timer::new(duration, false);
        cooldown.tick(duration);

        Self { logic, cooldown }
    }
    fn cast(&mut self) {
        if self.cooldown.finished() {
            self.cooldown.reset();
            (self.logic)();
        }
    }
}

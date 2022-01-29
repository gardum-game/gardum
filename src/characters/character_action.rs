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
use leafwing_input_manager::prelude::*;
use strum::EnumIter;

use crate::core::{AppState, Local};

pub(super) struct CharacterActionPlugin;

impl Plugin for CharacterActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_actions));
    }
}

/// Setup player input on game start
fn setup_actions(mut commands: Commands, local_player: Query<Entity, With<Local>>) {
    let local_player = local_player.single();
    let mut input_map = InputMap::default();
    input_map
        .insert(CharacterAction::Forward, KeyCode::W)
        .insert(CharacterAction::Backward, KeyCode::S)
        .insert(CharacterAction::Left, KeyCode::A)
        .insert(CharacterAction::Right, KeyCode::D)
        .insert(CharacterAction::Jump, KeyCode::Space)
        .insert(CharacterAction::BaseAttack, MouseButton::Left)
        .insert(CharacterAction::Ability1, KeyCode::Q)
        .insert(CharacterAction::Ability2, KeyCode::E)
        .insert(CharacterAction::Ability3, KeyCode::LShift)
        .insert(CharacterAction::Ultimate, KeyCode::R);
    commands.entity(local_player).insert(input_map);
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
pub(crate) enum CharacterAction {
    // Movement
    Forward,
    Backward,
    Left,
    Right,
    Jump,

    // Abilities activation
    BaseAttack,
    Ability1,
    Ability2,
    Ability3,
    Ultimate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mappings_setup() {
        let mut app = setup_app();
        let player = app.world.spawn().insert(Local).id();

        app.update();

        assert!(
            app.world
                .get_entity(player)
                .unwrap()
                .contains::<InputMap<CharacterAction>>(),
            "Mappings should be added to the local player"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(AppState::InGame)
            .add_plugin(CharacterActionPlugin);
        app
    }
}

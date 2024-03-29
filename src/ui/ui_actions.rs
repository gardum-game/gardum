/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct UiActionsPlugin;

impl Plugin for UiActionsPlugin {
    fn build(&self, app: &mut App) {
        let mut input_map = InputMap::default();
        input_map
            .insert(KeyCode::Escape, UiAction::Back)
            .insert(KeyCode::Tab, UiAction::Scoreboard)
            .insert(KeyCode::Return, UiAction::Chat);

        app.init_resource::<ActionState<UiAction>>()
            .insert_resource(input_map);
    }
}

#[derive(Actionlike, Clone, Copy)]
pub(crate) enum UiAction {
    Back,
    Scoreboard,
    Chat,
}

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
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};
use leafwing_input_manager::prelude::ActionState;

use super::{ui_actions::UiAction, ui_state::UiState, UI_MARGIN};
use crate::core::game_state::GameState;

pub(super) struct BackButtonPlugin;

impl Plugin for BackButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::SettingsMenu).with_system(Self::back_button_system),
        )
        .add_system_set(
            SystemSet::on_update(UiState::ServerBrowser).with_system(Self::back_button_system),
        )
        .add_system_set(
            SystemSet::on_update(UiState::DirectConnectMenu).with_system(Self::back_button_system),
        );
    }
}

impl BackButtonPlugin {
    pub(super) fn back_button_system(
        game_state: Res<State<GameState>>,
        mut action_state: ResMut<ActionState<UiAction>>,
        egui: ResMut<EguiContext>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        Area::new("Back area")
            .anchor(Align2::LEFT_BOTTOM, (UI_MARGIN, -UI_MARGIN))
            .show(egui.ctx(), |ui| {
                if action_state.just_pressed(UiAction::Back) || ui.button("Back").clicked() {
                    let previous_state = match ui_state.current() {
                        UiState::ServerBrowser => UiState::MainMenu,
                        UiState::SettingsMenu => match game_state.current() {
                            GameState::Menu => UiState::MainMenu,
                            GameState::InGame => UiState::InGameMenu,
                        },
                        UiState::DirectConnectMenu => UiState::ServerBrowser,
                        _ => unreachable!("Previous state isn't defined for this state"),
                    };

                    ui_state.set(previous_state).unwrap();
                    action_state.consume(UiAction::Back);
                }
            });
    }
}

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

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};
use leafwing_input_manager::prelude::ActionState;

use crate::core::game_state::GameState;

use super::{chat::ChatPlugin, ui_actions::UiAction, ui_state::UiState};

pub(super) struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::InGameMenu)
                .with_system(Self::ingame_menu_system)
                .with_system(Self::hide_ingame_menu_system.after(ChatPlugin::chat_system)),
        );
    }
}

impl InGameMenuPlugin {
    fn ingame_menu_system(
        mut exit_event: EventWriter<AppExit>,
        mut egui: ResMut<EguiContext>,
        mut ui_state: ResMut<State<UiState>>,
        mut game_state: ResMut<State<GameState>>,
    ) {
        Area::new("Main Menu")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .show(egui.ctx_mut(), |ui| {
                if ui.button("Resume").clicked() {
                    ui_state.set(UiState::Hud).unwrap();
                }
                if ui.button("Settings").clicked() {
                    ui_state.set(UiState::SettingsMenu).unwrap();
                }
                if ui.button("Main menu").clicked() {
                    ui_state.set(UiState::MainMenu).unwrap();
                    game_state.set(GameState::Menu).unwrap();
                }
                if ui.button("Exit").clicked() {
                    exit_event.send(AppExit);
                }
            });
    }

    fn hide_ingame_menu_system(
        mut action_state: ResMut<ActionState<UiAction>>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        if action_state.just_pressed(UiAction::Back) {
            action_state.consume(UiAction::Back);
            ui_state.set(UiState::Hud).unwrap();
        }
    }
}

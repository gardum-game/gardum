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
use bevy_egui::{
    egui::{Align2, TextEdit, Window},
    EguiContext,
};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::ui::{
    back_button::BackButton, chat_window::ChatWindowPlugin, ui_actions::UiAction, ui_state::UiState,
};

pub(super) struct ServerBrowserPlugin;

impl Plugin for ServerBrowserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SearchText>()
            .add_system(Self::game_browser_system.run_in_state(UiState::ServerBrowser))
            .add_system(
                Self::back_system
                    .run_in_state(UiState::ServerBrowser)
                    .after(ChatWindowPlugin::chat_system),
            );
    }
}

impl ServerBrowserPlugin {
    fn game_browser_system(
        mut commands: Commands,
        mut search_text: Local<SearchText>,
        mut egui: ResMut<EguiContext>,
    ) {
        Window::new("Game browser")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.add_enabled(
                        false,
                        TextEdit::singleline(&mut search_text.0).hint_text("Search servers"),
                    );
                    if ui.button("Connect").clicked() {
                        commands.insert_resource(NextState(UiState::DirectConnectMenu));
                    }
                    if ui.button("Create").clicked() {
                        commands.insert_resource(NextState(UiState::LobbyMenu));
                    }
                });
                ui.add_space(ui.available_height());
            });
    }

    fn back_system(
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        if BackButton::new(&mut action_state)
            .show(egui.ctx_mut())
            .clicked()
        {
            commands.insert_resource(NextState(UiState::MainMenu));
        }
    }
}

#[derive(Default)]
struct SearchText(String);

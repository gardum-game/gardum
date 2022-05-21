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
    egui::{Align2, TextEdit, Window},
    EguiContext,
};

use crate::ui::ui_state::UiState;

pub(super) struct ServerBrowserPlugin;

impl Plugin for ServerBrowserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SearchText>().add_system_set(
            SystemSet::on_update(UiState::ServerBrowser).with_system(Self::game_browser_system),
        );
    }
}

impl ServerBrowserPlugin {
    fn game_browser_system(
        mut search_text: Local<SearchText>,
        mut egui: ResMut<EguiContext>,
        mut ui_state: ResMut<State<UiState>>,
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
                        ui_state.set(UiState::DirectConnectMenu).unwrap();
                    }
                    if ui.button("Create").clicked() {
                        ui_state.set(UiState::LobbyMenu).unwrap();
                    }
                });
                ui.add_space(ui.available_height());
            });
    }
}

#[derive(Default)]
struct SearchText(String);

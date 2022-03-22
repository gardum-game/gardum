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
    egui::{Align2, Grid, Window},
    EguiContext,
};

use crate::{core::game_state::GameState, ui::ui_state::UiState};

pub(super) struct DirectConnectMenuPlugin;

impl Plugin for DirectConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::DirectConnectMenu)
                .with_system(direct_connect_menu_system),
        );
    }
}

struct DirectConnectData {
    ip: String,
    port: String,
}

impl Default for DirectConnectData {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: "4761".to_string(),
        }
    }
}

fn direct_connect_menu_system(
    egui: ResMut<EguiContext>,
    mut data: Local<DirectConnectData>,
    mut game_state: ResMut<State<GameState>>,
) {
    Window::new("Direct connect")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            Grid::new("Direct connect grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("IP:");
                    ui.text_edit_singleline(&mut data.ip);
                    ui.end_row();
                    ui.label("Port:");
                    ui.text_edit_singleline(&mut data.port);
                    ui.end_row();
                });
            ui.vertical_centered(|ui| {
                if ui.button("Connect").clicked() {
                    game_state.set(GameState::InGame).unwrap();
                }
            });
        });
}

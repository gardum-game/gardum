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
    egui::{Align2, DragValue, Grid, Window},
    EguiContext,
};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    core::network::client::ConnectionSettings,
    ui::{
        back_button::BackButton, chat::ChatPlugin, error_dialog::ErrorDialog, ui_actions::UiAction,
        ui_state::UiState,
    },
};

pub(super) struct DirectConnectMenuPlugin;

impl Plugin for DirectConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::DirectConnectMenu)
                .with_system(DirectConnectMenuPlugin::direct_connect_menu_system)
                .with_system(Self::back_system.after(ChatPlugin::chat_system)),
        );
    }
}

impl DirectConnectMenuPlugin {
    fn direct_connect_menu_system(
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        mut connection_setttings: ResMut<ConnectionSettings>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        Window::new("Direct connect")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
                Grid::new("Direct connect grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("IP:");
                        ui.text_edit_singleline(&mut connection_setttings.ip);
                        ui.end_row();
                        ui.label("Port:");
                        ui.add(
                            DragValue::new(&mut connection_setttings.port).clamp_range(0..=65535),
                        );
                        ui.end_row();
                    });
                ui.vertical_centered(|ui| {
                    if ui.button("Connect").clicked() {
                        match connection_setttings.create_client() {
                            Ok(client) => {
                                ui_state.set(UiState::ConnectionDialog).unwrap();
                                commands.insert_resource(client);
                            }
                            Err(error) => commands.insert_resource(ErrorDialog {
                                title: "Unable to create connection".to_string(),
                                text: error.to_string(),
                            }),
                        }
                    }
                });
            });
    }

    fn back_system(
        mut egui: ResMut<EguiContext>,
        mut action_state: ResMut<ActionState<UiAction>>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        if BackButton::new(&mut action_state)
            .show(egui.ctx_mut())
            .clicked()
        {
            ui_state.set(UiState::ServerBrowser).unwrap();
        }
    }
}

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
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    core::network::{client::ConnectionSettings, NetworkingState, MAX_PORT},
    ui::{
        back_button::BackButton, chat_window::ChatWindowPlugin, error_dialog::ErrorMessage,
        modal_window::ModalWindow, ui_actions::UiAction, ui_state::UiState,
    },
};

pub(super) struct DirectConnectMenuPlugin;

impl Plugin for DirectConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::direct_connect_menu_system.run_in_state(UiState::DirectConnectMenu))
            .add_system(
                Self::connection_dialog_system
                    .run_in_state(NetworkingState::Connecting)
                    .run_in_state(UiState::DirectConnectMenu),
            )
            .add_system(
                Self::enter_lobby_system
                    .run_in_state(NetworkingState::Connected)
                    .run_in_state(UiState::DirectConnectMenu),
            )
            .add_system(
                Self::back_system
                    .run_in_state(UiState::DirectConnectMenu)
                    .after(ChatWindowPlugin::chat_system),
            );
    }
}

impl DirectConnectMenuPlugin {
    fn direct_connect_menu_system(
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        mut connection_setttings: ResMut<ConnectionSettings>,
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
                            DragValue::new(&mut connection_setttings.port)
                                .clamp_range(0..=MAX_PORT),
                        );
                        ui.end_row();
                    });
                ui.vertical_centered(|ui| {
                    if ui.button("Connect").clicked() {
                        match connection_setttings.create_client() {
                            Ok(client) => commands.insert_resource(client),
                            Err(error) => commands.insert_resource(ErrorMessage {
                                title: "Unable to create connection".to_string(),
                                text: error.to_string(),
                            }),
                        }
                    }
                });
            });
    }

    fn connection_dialog_system(
        mut commands: Commands,
        connection_setttings: Res<ConnectionSettings>,
        mut egui: ResMut<EguiContext>,
    ) {
        ModalWindow::new("Connecting").show(egui.ctx_mut(), |ui| {
            ui.label(format!(
                "Connecting to {}:{}...",
                connection_setttings.ip, connection_setttings.port
            ));
            if ui.button("Cancel").clicked() {
                commands.insert_resource(NextState(NetworkingState::NoSocket));
            }
        });
    }

    fn enter_lobby_system(mut commands: Commands) {
        commands.insert_resource(NextState(UiState::LobbyMenu));
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
            commands.insert_resource(NextState(UiState::ServerBrowser));
        }
    }
}

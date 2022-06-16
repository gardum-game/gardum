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
use bevy_egui::EguiContext;
use iyes_loopless::prelude::*;

use super::{modal_window::ModalWindow, ui_state::UiState};

pub(super) struct ErrorDialogPlugin;

impl Plugin for ErrorDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            Self::error_dialog_system
                .run_if_resource_exists::<ErrorMessage>()
                .run_in_state(UiState::LobbyMenu),
        )
        .add_system(
            Self::error_dialog_system
                .run_if_resource_exists::<ErrorMessage>()
                .run_in_state(UiState::DirectConnectMenu),
        );
    }
}

impl ErrorDialogPlugin {
    fn error_dialog_system(
        mut commands: Commands,
        error_message: Res<ErrorMessage>,
        mut egui: ResMut<EguiContext>,
    ) {
        ModalWindow::new(&error_message.title).show(egui.ctx_mut(), |ui| {
            ui.label(&error_message.text);
            if ui.button("Ok").clicked() {
                commands.remove_resource::<ErrorMessage>();
            }
        });
    }
}

pub(super) struct ErrorMessage {
    pub(super) title: String,
    pub(super) text: String,
}

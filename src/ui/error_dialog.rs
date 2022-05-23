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
use bevy_egui::EguiContext;

use super::{modal_window::ModalWindow, ui_state::UiState};

pub(super) struct ErrorDialogPlugin;

impl Plugin for ErrorDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::LobbyMenu).with_system(Self::error_dialog_system),
        )
        .add_system_set(
            SystemSet::on_update(UiState::DirectConnectMenu).with_system(Self::error_dialog_system),
        );
    }
}

impl ErrorDialogPlugin {
    fn error_dialog_system(
        mut commands: Commands,
        error_message: Option<Res<ErrorDialog>>,
        mut egui: ResMut<EguiContext>,
    ) {
        if let Some(error_message) = error_message {
            ModalWindow::new(&error_message.title).show(egui.ctx_mut(), |ui| {
                ui.label(&error_message.text);
                if ui.button("Ok").clicked() {
                    commands.remove_resource::<ErrorDialog>();
                }
            });
        }
    }
}

pub(super) struct ErrorDialog {
    pub(super) title: String,
    pub(super) text: String,
}

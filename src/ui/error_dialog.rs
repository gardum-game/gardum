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
    egui::{Align2, Window},
    EguiContext,
};

use super::ui_state::UiState;

pub(super) struct ErrorDialogPlugin;

impl Plugin for ErrorDialogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ErrorDialog>().add_system_set(
            SystemSet::on_update(UiState::ErrorDialog).with_system(Self::error_dialog_system),
        );
    }
}

impl ErrorDialogPlugin {
    fn error_dialog_system(
        error_dialog: Res<ErrorDialog>,
        egui: Res<EguiContext>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        Window::new(&error_dialog.title)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx(), |ui| {
                ui.label(&error_dialog.message);
                if ui.button("Ok").clicked() {
                    ui_state.set(error_dialog.previous_state).unwrap();
                }
            });
    }
}

#[derive(Default)]
pub(super) struct ErrorDialog {
    title: String,
    message: String,
    previous_state: UiState,
}

impl ErrorDialog {
    pub(super) fn show(&mut self, title: String, message: String, ui_state: &mut State<UiState>) {
        self.title = title;
        self.message = message;
        self.previous_state = *ui_state.current();
        ui_state.set(UiState::ErrorDialog).unwrap();
    }
}

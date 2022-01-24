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

use super::{
    ui_state::{UiState, UiStateHistory},
    UI_MARGIN,
};

pub(super) struct BackButtonPlugin;

impl Plugin for BackButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::CustomGameMenu).with_system(back_button_system),
        )
        .add_system_set(
            SystemSet::on_update(UiState::DirectConnectMenu).with_system(back_button_system),
        )
        .add_system_set(
            SystemSet::on_update(UiState::CreateGameMenu).with_system(back_button_system),
        )
        .add_system_set(SystemSet::on_update(UiState::LobbyMenu).with_system(back_button_system));
    }
}

fn back_button_system(
    egui: ResMut<EguiContext>,
    input: Res<Input<KeyCode>>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    Area::new("Back area")
        .anchor(Align2::LEFT_BOTTOM, (UI_MARGIN, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            if input.just_pressed(KeyCode::Escape) || ui.button("Back").clicked() {
                ui_state_history.pop();
            }
        });
}
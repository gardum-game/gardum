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
    egui::{Align2, Area, ComboBox, Grid, Window},
    EguiContext,
};

use super::{
    ui_state::{UiState, UiStateHistory},
    UI_MARGIN,
};
use crate::core::settings::{SettingApplyEvent, Settings};

pub(super) struct SettingMenuPlugin;

impl Plugin for SettingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::SettingsMenu).with_system(settings_menu_system),
        );
    }
}

fn settings_menu_system(
    egui: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut settings: ResMut<Settings>,
    mut apply_events: EventWriter<SettingApplyEvent>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    let main_window = windows.get_primary().unwrap();

    Window::new("Settings")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .vscroll(true)
        .show(egui.ctx(), |ui| {
            Grid::new("Settings grid")
                .num_columns(2)
                .striped(true)
                .min_col_width(main_window.width() / 2.0)
                .show(ui, |ui| {
                    ui.label("MSAA");
                    ComboBox::from_id_source("MSAA combobox")
                        .selected_text(settings.video.msaa.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut settings.video.msaa, 1, 1.to_string());
                            ui.selectable_value(&mut settings.video.msaa, 4, 4.to_string());
                        });
                    ui.end_row();
                });
            ui.add_space(ui.available_height());
        });

    Area::new("Settings buttons area")
        .anchor(Align2::RIGHT_BOTTOM, (-UI_MARGIN, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Restore defaults").clicked() {
                    *settings = Settings::default();
                    apply_events.send(SettingApplyEvent);
                }
                if ui.button("Apply").clicked() {
                    apply_events.send(SettingApplyEvent);
                }
                if ui.button("Ok").clicked() {
                    apply_events.send(SettingApplyEvent);
                    ui_state_history.pop();
                }
            })
        });
}

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
    egui::{Align2, Area, ComboBox, Ui, Window},
    EguiContext,
};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::{
    ui_state::{UiState, UiStateHistory},
    UI_MARGIN,
};
use crate::core::settings::{SettingApplyEvent, Settings, VideoSettings};

pub(super) struct SettingMenuPlugin;

impl Plugin for SettingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::SettingsMenu)
                .with_system(settings_menu_system)
                .with_system(settings_buttons_system),
        );
    }
}

fn settings_menu_system(
    egui: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut settings: ResMut<Settings>,
    mut current_tab: Local<SettingsTab>,
) {
    let main_window = windows.get_primary().unwrap();
    let window_margin = egui.ctx().style().spacing.window_margin.left;

    Window::new("Settings")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .default_width(main_window.width() - UI_MARGIN * 2.0 - window_margin * 2.0)
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                for tab in SettingsTab::iter() {
                    ui.selectable_value(&mut *current_tab, tab, tab.to_string());
                }
            });
            match *current_tab {
                SettingsTab::Video => show_video_settings(ui, &mut settings.video),
            };
            ui.expand_to_include_rect(ui.available_rect_before_wrap());
        });
}

fn settings_buttons_system(
    egui: ResMut<EguiContext>,
    mut apply_events: EventWriter<SettingApplyEvent>,
    mut settings: ResMut<Settings>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
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

fn show_video_settings(ui: &mut Ui, video_settings: &mut VideoSettings) {
    ComboBox::from_label("MSAA samples")
        .selected_text(video_settings.msaa.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut video_settings.msaa, 1, 1.to_string());
            ui.selectable_value(&mut video_settings.msaa, 4, 4.to_string());
        });
}

#[derive(Display, Clone, Copy, EnumIter, PartialEq)]
enum SettingsTab {
    Video,
}

impl Default for SettingsTab {
    fn default() -> Self {
        SettingsTab::Video
    }
}

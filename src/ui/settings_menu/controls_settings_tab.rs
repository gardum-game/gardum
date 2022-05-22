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

use bevy::prelude::Commands;
use bevy_egui::egui::{Grid, Ui};
use leafwing_input_manager::{
    user_input::{InputButton, UserInput},
    Actionlike,
};

use crate::{
    core::{control_actions::ControlAction, settings::ControlsSettings},
    ui::settings_menu::ActiveBinding,
};

pub(super) struct ControlsSettingsTab<'a> {
    controls_settings: &'a mut ControlsSettings,
}

impl<'a> ControlsSettingsTab<'a> {
    #[must_use]
    pub(super) fn new(controls_settings: &'a mut ControlsSettings) -> Self {
        Self { controls_settings }
    }
}

impl ControlsSettingsTab<'_> {
    pub(super) fn show(self, ui: &mut Ui, commands: &mut Commands) {
        const INPUT_VARIANTS: usize = 3;
        const COLUMNS_COUNT: usize = INPUT_VARIANTS + 1;
        let window_width_margin = ui.style().spacing.window_margin.left * 2.0;

        Grid::new("Controls grid")
            .num_columns(COLUMNS_COUNT)
            .striped(true)
            .min_col_width(ui.available_width() / COLUMNS_COUNT as f32 - window_width_margin)
            .show(ui, |ui| {
                for action in ControlAction::variants() {
                    ui.label(action.to_string());
                    let inputs = self.controls_settings.mappings.get(action);
                    for index in 0..INPUT_VARIANTS {
                        let button_text = match inputs.get_at(index) {
                            Some(UserInput::Single(InputButton::Gamepad(gamepad_button))) => {
                                format!("🎮 {:?}", gamepad_button)
                            }
                            Some(UserInput::Single(InputButton::Keyboard(keycode))) => {
                                format!("🖮 {:?}", keycode)
                            }
                            Some(UserInput::Single(InputButton::Mouse(mouse_button))) => {
                                format!("🖱 {:?}", mouse_button)
                            }
                            _ => "Empty".to_string(),
                        };
                        if ui.button(button_text).clicked() {
                            commands.insert_resource(ActiveBinding::new(action, index));
                        }
                    }
                    ui.end_row();
                }
            });
    }
}

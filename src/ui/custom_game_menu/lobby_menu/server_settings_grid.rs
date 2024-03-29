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

use bevy_egui::egui::{Checkbox, ComboBox, DragValue, Grid, TextEdit, Ui};
use strum::IntoEnumIterator;

use crate::core::{map::Map, network::server::ServerSettings, session::GameMode};

/// Displays ability icon and its cooldown.
pub(super) struct GameSettingsGrid<'a> {
    server_settings: &'a mut ServerSettings,
    editable: bool,
}

impl<'a> GameSettingsGrid<'a> {
    pub(super) fn new(server_settings: &'a mut ServerSettings, editable: bool) -> Self {
        Self {
            server_settings,
            editable,
        }
    }
}

impl GameSettingsGrid<'_> {
    pub(super) fn show(self, ui: &mut Ui) {
        Grid::new("Server settings grid").show(ui, |ui| {
            ui.heading("Settings");
            ui.end_row();
            ui.label("Server name:");
            ui.add_enabled(
                self.editable,
                TextEdit::singleline(&mut self.server_settings.server_name),
            );
            ui.end_row();
            ui.label("Port:");
            ui.add_enabled(
                self.editable,
                DragValue::new(&mut self.server_settings.port),
            );
            ui.end_row();
            ui.label("Game mode:");
            ui.add_enabled_ui(self.editable, |ui| {
                ComboBox::from_id_source("Game mode")
                    .selected_text(format!("{:?}", &mut self.server_settings.game_mode))
                    .show_ui(ui, |ui| {
                        for game_mode in GameMode::iter() {
                            ui.selectable_value(
                                &mut self.server_settings.game_mode,
                                game_mode,
                                format!("{:?}", game_mode),
                            );
                        }
                    });
            });
            ui.end_row();
            ui.label("Map:");
            ui.add_enabled_ui(self.editable, |ui| {
                ComboBox::from_id_source("Map")
                    .selected_text(format!("{:?}", &mut self.server_settings.map))
                    .show_ui(ui, |ui| {
                        for map in Map::iter() {
                            ui.selectable_value(
                                &mut self.server_settings.map,
                                map,
                                format!("{:?}", map),
                            );
                        }
                    });
            });
            ui.end_row();
            ui.add_enabled(
                self.editable,
                Checkbox::new(&mut self.server_settings.random_heroes, "Random heroes:"),
            );
            ui.end_row();
        });
    }
}

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

use bevy_egui::egui::Ui;

use crate::core::settings::DeveloperSettings;

pub(super) struct DeveloperTab<'a> {
    developer_settings: &'a mut DeveloperSettings,
}

impl<'a> DeveloperTab<'a> {
    #[must_use]
    pub(super) fn new(developer_settings: &'a mut DeveloperSettings) -> Self {
        Self { developer_settings }
    }
}

impl DeveloperTab<'_> {
    pub(super) fn show(self, ui: &mut Ui) {
        ui.checkbox(
            &mut self.developer_settings.world_inspector,
            "Enable world inspector",
        );
        ui.checkbox(
            &mut self.developer_settings.debug_collisions,
            "Debug collisions",
        );
    }
}

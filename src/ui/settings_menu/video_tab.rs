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

use bevy_egui::egui::{ComboBox, Ui};

use crate::core::settings::VideoSettings;

pub(super) struct VideoTab<'a> {
    video_settings: &'a mut VideoSettings,
}

impl<'a> VideoTab<'a> {
    #[must_use]
    pub(super) fn new(video_settings: &'a mut VideoSettings) -> Self {
        Self { video_settings }
    }
}

impl VideoTab<'_> {
    pub(super) fn show(self, ui: &mut Ui) {
        ComboBox::from_label("MSAA samples")
            .selected_text(self.video_settings.msaa.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.video_settings.msaa, 1, 1.to_string());
                ui.selectable_value(&mut self.video_settings.msaa, 4, 4.to_string());
            });
        ui.checkbox(
            &mut self.video_settings.perf_stats,
            "Display performance stats",
        );
    }
}

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

use bevy_egui::egui::{Color32, Response, ScrollArea, TextEdit, TextStyle, Ui, Widget};

use super::Chat;

pub(super) struct MessagesArea<'a> {
    chat: &'a mut Chat,
}

impl<'a> MessagesArea<'a> {
    #[must_use]
    pub(super) fn new(chat: &'a mut Chat) -> Self {
        Self { chat }
    }
}

impl<'a> Widget for MessagesArea<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let extreme_bg_color = ui.visuals().extreme_bg_color;
        let bg_fill = ui.visuals().widgets.inactive.bg_fill;
        if !self.chat.active {
            // Hide scrollbar
            ui.visuals_mut().extreme_bg_color = Color32::TRANSPARENT;
            ui.visuals_mut().widgets.inactive.bg_fill = Color32::TRANSPARENT;
        }

        const CHAT_ROWS_MARGIN: f32 = 4.0;
        const CHAT_VISIBLE_ROWS: usize = 8;
        let text_height = ui.text_style_height(&TextStyle::Body);
        let scroll_response = ScrollArea::vertical()
            .max_height(text_height * CHAT_VISIBLE_ROWS as f32 + CHAT_ROWS_MARGIN)
            .stick_to_bottom()
            .enable_scrolling(self.chat.active)
            .show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut self.chat.text.as_str())
                        .desired_rows(CHAT_VISIBLE_ROWS),
                )
            });

        if !self.chat.active {
            // Restore style after hiding scrollbar
            ui.visuals_mut().extreme_bg_color = extreme_bg_color;
            ui.visuals_mut().widgets.inactive.bg_fill = bg_fill;
        }

        scroll_response.inner
    }
}

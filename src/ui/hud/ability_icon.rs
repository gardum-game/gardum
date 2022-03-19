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

use bevy_egui::egui::*;

use crate::core::cooldown::Cooldown;

/// Displays ability icon and its cooldown.
pub(super) struct AbilityIcon<'a> {
    texture_id: TextureId,
    cooldown: Option<&'a Cooldown>,
}

impl<'a> AbilityIcon<'a> {
    /// `current` shouldn't be bigger then `max`
    pub(super) fn new(texture_id: TextureId, cooldown: Option<&'a Cooldown>) -> Self {
        Self {
            cooldown,
            texture_id,
        }
    }
}

impl Widget for AbilityIcon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let AbilityIcon {
            texture_id,
            cooldown,
        } = self;
        let image = Image::new(texture_id, [64.0, 64.0]);

        let (rect, response) = ui.allocate_at_least(
            image.size(),
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );

        image.paint_at(ui, rect);

        if let Some(cooldown) = cooldown {
            let display_sec = (cooldown.duration().as_secs_f32() - cooldown.elapsed_secs()).ceil();
            if display_sec != 0.0 {
                let fade_rect = Rect::from_min_size(
                    rect.min,
                    vec2(rect.size().x, rect.size().y * cooldown.percent_left()),
                );
                ui.painter().rect(
                    fade_rect,
                    0.0,
                    Color32::from_black_alpha(150),
                    Stroke::none(),
                );

                let text: WidgetText = RichText::new(display_sec.to_string())
                    .font(FontId::monospace(25.0))
                    .strong()
                    .color(Color32::DARK_GRAY)
                    .into();
                let text_galley = text.into_galley(ui, None, f32::INFINITY, TextStyle::Heading);
                let text_pos = rect.center() - text_galley.size() / 2.0;
                ui.painter()
                    .sub_region(rect)
                    .galley(text_pos, text_galley.galley);
            }
        }

        response
    }
}

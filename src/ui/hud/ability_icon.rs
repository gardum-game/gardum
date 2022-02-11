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

use crate::core::ability::Cooldown;

/// Displays ability icon and its cooldown.
pub(super) struct AbilityIcon<'a> {
    image: Image,
    cooldown: Option<&'a Cooldown>,
}

impl<'a> AbilityIcon<'a> {
    /// `current` shouldn't be bigger then `max`
    pub(super) fn new(image: Image, cooldown: Option<&'a Cooldown>) -> Self {
        Self { cooldown, image }
    }
}

impl Widget for AbilityIcon<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let AbilityIcon { image, cooldown } = self;

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
                    vec2(
                        rect.size().x,
                        rect.size().y
                            * (1.0 - cooldown.elapsed_secs() / cooldown.duration().as_secs_f32()),
                    ),
                );
                ui.painter().rect(
                    fade_rect,
                    0.0,
                    Color32::from_black_alpha(150),
                    Stroke::none(),
                );
                let text: WidgetText = display_sec.to_string().into();
                let galley = text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Heading);
                let text_pos = rect.center() - galley.size() / 2.0;
                let text_color = Color32::DARK_GRAY;
                galley.paint_with_fallback_color(
                    &ui.painter().sub_region(rect),
                    text_pos,
                    text_color,
                );
            }
        }

        response
    }
}

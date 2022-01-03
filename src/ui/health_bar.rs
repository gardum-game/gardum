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

/// A simple health bar.
pub struct HealthBar {
    current: u32,
    max: u32,
}

impl HealthBar {
    /// `current` shouldn't be bigger then `max`
    pub fn new(current: u32, max: u32) -> Self {
        Self { current, max }
    }
}

impl Widget for HealthBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let HealthBar { current, max } = self;

        let height = ui.spacing().interact_size.y;
        let (outer_rect, response) = ui.allocate_exact_size(
            vec2(ui.available_size_before_wrap().x, height),
            Sense::hover(),
        );

        if ui.is_rect_visible(response.rect) {
            let visuals = ui.style().visuals.clone();
            ui.painter()
                .rect(outer_rect, 0.0, visuals.extreme_bg_color, Stroke::none());

            let inner_rect = Rect::from_min_size(
                outer_rect.min,
                vec2(
                    outer_rect.width() * current as f32 / max as f32,
                    outer_rect.height(),
                ),
            );
            ui.painter()
                .rect(inner_rect, 0.0, Color32::DARK_GREEN, Stroke::none());

            let text: WidgetText = format!("{} / {}", current, max).into();
            let galley = text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Button);
            let text_pos = outer_rect.left_center() - Vec2::new(0.0, galley.size().y / 2.0)
                + vec2(ui.spacing().item_spacing.x, 0.0);
            let text_color = visuals
                .override_text_color
                .unwrap_or(visuals.selection.stroke.color);
            galley.paint_with_fallback_color(
                &ui.painter().sub_region(outer_rect),
                text_pos,
                text_color,
            );
        }

        response
    }
}

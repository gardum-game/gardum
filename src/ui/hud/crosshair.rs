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

use bevy_egui::egui::*;

/// Displays ability icon and its cooldown.
pub(super) struct Crosshair {
    radius: f32,
    thickness: f32,
    dot_radius: f32,
    color: Color32,
}

impl Default for Crosshair {
    fn default() -> Self {
        Self {
            radius: 20.0,
            thickness: 1.0,
            dot_radius: 2.0,
            color: Color32::from_white_alpha(128),
        }
    }
}

impl Widget for Crosshair {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = Vec2::splat(2.0 * self.radius + self.thickness);
        let (response, painter) = ui.allocate_painter(size, Sense::hover());

        if ui.is_rect_visible(response.rect) {
            painter.circle_stroke(
                response.rect.center(),
                self.radius,
                Stroke::new(self.thickness, self.color),
            );
            painter.circle_filled(response.rect.center(), self.dot_radius, self.color);
        }

        response
    }
}

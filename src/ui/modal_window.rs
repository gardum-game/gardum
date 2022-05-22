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

use bevy_egui::egui::{
    Align2, Area, Color32, Context, InnerResponse, Pos2, Shape, Ui, WidgetText, Window,
};

pub(super) struct ModalWindow<'a> {
    window: Window<'a>,
}

impl ModalWindow<'_> {
    #[must_use]
    pub(super) fn new(title: impl Into<WidgetText>) -> Self {
        Self {
            window: Window::new(title),
        }
    }
}

impl ModalWindow<'_> {
    pub fn show<R>(
        self,
        ctx: &Context,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<Option<R>> {
        // Create an area to prevent interation with other widgets
        // and display semi-transparent background
        const BACKGROUND_ALPHA: u8 = 200;
        Area::new("Modal area")
            .fixed_pos(Pos2::ZERO)
            .show(ctx, |ui| {
                let screen = ui.ctx().input().screen_rect();
                ui.painter().add(Shape::rect_filled(
                    screen,
                    0.0,
                    Color32::from_black_alpha(BACKGROUND_ALPHA),
                ));
                ui.allocate_space(screen.size());
            });

        let inner_response = self
            .window
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, add_contents)
            .unwrap();

        ctx.move_to_top(inner_response.response.layer_id);
        inner_response
    }
}

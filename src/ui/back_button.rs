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

use bevy_egui::egui::{Align2, Area, Context, PointerButton, Response};
use leafwing_input_manager::prelude::*;

use super::{ui_actions::UiAction, UI_MARGIN};

pub(super) struct BackButton<'a> {
    action_state: &'a mut ActionState<UiAction>,
}

impl<'a> BackButton<'a> {
    #[must_use]
    pub(super) fn new(action_state: &'a mut ActionState<UiAction>) -> Self {
        Self { action_state }
    }

    #[must_use]
    pub(super) fn show(self, ctx: &Context) -> Response {
        Area::new("Back area")
            .anchor(Align2::LEFT_BOTTOM, (UI_MARGIN, -UI_MARGIN))
            .show(ctx, |ui| {
                let mut response = ui.button("Back");
                if !response.clicked() && self.action_state.just_pressed(UiAction::Back) {
                    // Count action as click
                    self.action_state.consume(UiAction::Back);
                    response.clicked[PointerButton::Primary as usize] = true;
                }
                response
            })
            .inner
    }
}

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

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Area, Color32, Stroke, TextEdit},
    EguiContext,
};
use leafwing_input_manager::{plugin::ToggleActions, prelude::ActionState};

use super::{
    back_button::BackButtonsSystems, ingame_menu::InGameMenuSystems, ui_state::UiState, UiAction,
    UI_MARGIN,
};
use crate::core::settings::ControlAction;

const CHAT_BOTTOM_MARGIN: f32 = 40.0;

pub(super) struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chat>()
            .add_system(
                chat_system
                    .before(BackButtonsSystems::BackButton)
                    .before(InGameMenuSystems::ShowInGameMenu)
                    .before(InGameMenuSystems::HideInGameMenu),
            )
            .add_system_set(SystemSet::on_update(UiState::Hud).with_system(toggle_control_actions));
    }
}

#[derive(Default)]
struct Chat {
    text: String,
    current_message: String,
    active: bool,
}

fn chat_system(
    mut action_state: ResMut<ActionState<UiAction>>,
    mut chat: ResMut<Chat>,
    egui: ResMut<EguiContext>,
) {
    Area::new("Chat area")
        .anchor(
            Align2::LEFT_BOTTOM,
            [UI_MARGIN, -UI_MARGIN - CHAT_BOTTOM_MARGIN],
        )
        .show(egui.ctx(), |ui| {
            if !chat.active {
                ui.style_mut().visuals.extreme_bg_color = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.noninteractive.bg_stroke = Stroke::none();
            } else {
                ui.style_mut().visuals.extreme_bg_color = ui.style().visuals.code_bg_color;
            }

            ui.add(
                TextEdit::multiline(&mut chat.text)
                    .interactive(false)
                    .desired_rows(8),
            );

            if chat.active {
                let response = ui.text_edit_singleline(&mut chat.current_message);
                if action_state.just_pressed(UiAction::Chat) {
                    action_state.consume(UiAction::Chat);
                    let chat = &mut *chat; // Borrow from resource first
                    let message = chat.current_message.trim();
                    if !message.is_empty() {
                        chat.text.push('\n');
                        chat.text.push_str(message);
                    }
                    chat.current_message.clear();
                    chat.active = false;
                } else if response.lost_focus() || action_state.just_pressed(UiAction::Back) {
                    action_state.consume(UiAction::Back);
                    chat.active = false;
                } else {
                    response.request_focus();
                }
            } else if ui.button("Chat").clicked() || action_state.just_pressed(UiAction::Chat) {
                action_state.consume(UiAction::Chat);
                chat.active = true;
            }
        });
}

fn toggle_control_actions(
    mut toggle_actions: ResMut<ToggleActions<ControlAction>>,
    chat: Res<Chat>,
) {
    // When chat is active, control actions should be disabled.
    if chat.active == toggle_actions.enabled {
        toggle_actions.enabled = !chat.active;
    }
}

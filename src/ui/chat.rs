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
    egui::{Align2, Area, Color32, Frame, Response, ScrollArea, TextEdit, TextStyle, Ui},
    EguiContext,
};
use leafwing_input_manager::{plugin::ToggleActions, prelude::ActionState};

use super::{back_button, ingame_menu, ui_actions::UiAction, ui_state::UiState, UI_MARGIN};
use crate::core::control_actions::ControlAction;

pub(super) struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chat>()
            .add_system(
                chat_system
                    .before(back_button::back_button_system)
                    .before(ingame_menu::hide_ingame_menu_system)
                    .before(ingame_menu::show_ingame_menu_system),
            )
            .add_system_set(SystemSet::on_update(UiState::Hud).with_system(toggle_control_actions));
    }
}

#[derive(Default)]
struct Chat {
    text: String,
    active: bool,
}

impl Chat {
    fn add_message(&mut self, message: &str) {
        if !message.is_empty() {
            if !self.text.is_empty() {
                self.text.push('\n');
            }
            self.text.push_str(message);
        }
    }
}

#[derive(Default)]
struct InputField {
    message: String,
    request_focus: bool,
}

fn chat_system(
    mut action_state: ResMut<ActionState<UiAction>>,
    mut chat: ResMut<Chat>,
    mut input: Local<InputField>,
    egui: ResMut<EguiContext>,
) {
    const CHAT_BOTTOM_MARGIN: f32 = 40.0;

    Area::new("Chat area")
        .anchor(
            Align2::LEFT_BOTTOM,
            [UI_MARGIN, -UI_MARGIN - CHAT_BOTTOM_MARGIN],
        )
        .show(egui.ctx(), |ui| {
            let frame = if chat.active {
                Frame::window(ui.style())
            } else {
                // Show frame with window spacing, but without visuals
                Frame {
                    margin: ui.style().spacing.window_margin,
                    rounding: ui.style().visuals.window_rounding,
                    ..Frame::none()
                }
            };
            frame.show(ui, |ui| {
                if !chat.active {
                    // Hide scrollbar
                    ui.style_mut().visuals.extreme_bg_color = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                }

                let chat_response = show_chat(ui, &mut chat);

                if !chat.active {
                    // Reset style after hiding scrollbar
                    ui.reset_style();
                }

                if chat.active {
                    let input_response = ui.text_edit_singleline(&mut input.message);
                    if input.request_focus {
                        input_response.request_focus();
                        input.request_focus = false;
                    }

                    if action_state.just_pressed(UiAction::Chat) {
                        action_state.consume(UiAction::Chat);
                        if input_response.lost_focus() {
                            chat.add_message(input.message.trim());
                            input.message.clear();
                            chat.active = false;
                        } else {
                            input_response.request_focus();
                        }
                    } else if (!input_response.has_focus() && !chat_response.has_focus())
                        || action_state.just_pressed(UiAction::Back)
                    {
                        action_state.consume(UiAction::Back);
                        chat.active = false;
                    }
                } else if ui.button("Chat").clicked() || action_state.just_pressed(UiAction::Chat) {
                    action_state.consume(UiAction::Chat);
                    chat.active = true;
                    input.request_focus = true;
                }

                if chat_response.gained_focus() {
                    chat.active = true;
                }
            });
        });
}

fn show_chat(ui: &mut Ui, chat: &mut Chat) -> Response {
    const CHAT_VISIBLE_ROWS: usize = 8;
    const CHAT_ROWS_MARGIN: f32 = 4.0;
    let text_height = ui.text_style_height(&TextStyle::Body);
    ScrollArea::vertical()
        .max_height(text_height * CHAT_VISIBLE_ROWS as f32 + CHAT_ROWS_MARGIN)
        .stick_to_bottom()
        .enable_scrolling(chat.active)
        .show(ui, |ui| {
            ui.add(TextEdit::multiline(&mut chat.text.as_str()).desired_rows(CHAT_VISIBLE_ROWS))
        })
        .inner
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

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

mod messages_area;

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Frame, Id, TextEdit, Window},
    EguiContext,
};
use bevy_renet::renet::ServerEvent;
use leafwing_input_manager::{plugin::ToggleActions, prelude::ActionState};

use super::{ui_actions::UiAction, ui_state::UiState, UI_MARGIN};
use crate::core::control_actions::ControlAction;
use messages_area::MessagesArea;

pub(super) struct ChatWindowPlugin;

impl Plugin for ChatWindowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chat>()
            .add_system(Self::chat_system)
            .add_system(Self::announce_connected_system)
            .add_system_set(
                SystemSet::on_update(UiState::Hud).with_system(Self::toggle_controls_system),
            );
    }
}

impl ChatWindowPlugin {
    pub(super) fn chat_system(
        input_field_id: Local<InputFieldId>,
        mut input: Local<InputField>,
        mut action_state: ResMut<ActionState<UiAction>>,
        mut egui: ResMut<EguiContext>,
        mut chat: ResMut<Chat>,
    ) {
        const CHAT_BOTTOM_MARGIN: f32 = 40.0;

        let frame = if chat.active {
            Frame::window(&egui.ctx_mut().style())
        } else {
            // Show frame with window spacing, but without visuals
            Frame {
                inner_margin: egui.ctx_mut().style().spacing.window_margin,
                rounding: egui.ctx_mut().style().visuals.window_rounding,
                ..Frame::none()
            }
        };

        Window::new("Chat window")
            .frame(frame)
            .title_bar(false)
            .resizable(false)
            .anchor(
                Align2::LEFT_BOTTOM,
                [UI_MARGIN, -UI_MARGIN - CHAT_BOTTOM_MARGIN],
            )
            .show(egui.ctx_mut(), |ui| {
                let chat_response = ui.add(MessagesArea::new(&mut chat));
                if chat.active {
                    let input_response =
                        ui.add(TextEdit::singleline(&mut input.message).id(input_field_id.0));
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
                    ui.memory().request_focus(input_field_id.0);
                }

                if chat_response.gained_focus() {
                    chat.active = true;
                }
            });
    }

    fn toggle_controls_system(
        chat: Res<Chat>,
        mut toggle_actions: ResMut<ToggleActions<ControlAction>>,
        mut windows: ResMut<Windows>,
    ) {
        // When controls are enabled and chat is active, we should disable them
        if chat.active == toggle_actions.enabled {
            toggle_actions.enabled = !chat.active;

            let window = windows.primary_mut();
            window.set_cursor_lock_mode(!chat.active);
            window.set_cursor_visibility(chat.active);
        }
    }

    fn announce_connected_system(
        mut server_events: EventReader<ServerEvent>,
        mut chat: ResMut<Chat>,
    ) {
        for event in server_events.iter() {
            match event {
                ServerEvent::ClientConnected(id, _) => {
                    chat.add_message(format!("Client connected: {}", id).as_str());
                }
                ServerEvent::ClientDisconnected(id) => {
                    chat.add_message(format!("Client disconnected: {}", id).as_str());
                }
            }
        }
    }
}

#[derive(Default)]
pub(super) struct Chat {
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
pub(super) struct InputField {
    message: String,
}

pub(super) struct InputFieldId(Id);

impl Default for InputFieldId {
    fn default() -> Self {
        Self(Id::new("Input field"))
    }
}

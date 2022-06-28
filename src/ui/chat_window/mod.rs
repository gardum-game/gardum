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

mod messages_area;

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Frame, Id, TextEdit, Window},
    EguiContext,
};
use bevy_renet::renet::{RenetServer, ServerEvent};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use std::mem;

use super::{ui_actions::UiAction, ui_state::UiState, UI_MARGIN};
use crate::core::{
    control_actions::ControlAction,
    network::{
        client,
        message::{ClientMessage, ServerMessage},
    },
    player::Player,
    Authority,
};
use messages_area::MessagesArea;

pub(super) struct ChatWindowPlugin;

impl Plugin for ChatWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageAccepted>()
            .init_resource::<Chat>()
            .add_system(Self::chat_system)
            .add_system(Self::announce_connected_system)
            .add_system(Self::toggle_controls_system.run_in_state(UiState::Hud))
            .add_system(
                Self::send_message_system
                    .run_on_event::<MessageAccepted>()
                    .run_if(client::connected),
            )
            .add_system(
                Self::send_message_system
                    .run_on_event::<MessageAccepted>()
                    .run_if_resource_exists::<RenetServer>(),
            )
            .add_system(Self::receive_message_system.run_if(client::connected))
            .add_system(Self::receive_message_system.run_if_resource_exists::<RenetServer>());
    }
}

impl ChatWindowPlugin {
    pub(super) fn chat_system(
        input_field_id: Local<InputFieldId>,
        mut send_events: EventWriter<MessageAccepted>,
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
                    let input_response = ui
                        .add(TextEdit::singleline(&mut chat.current_message).id(input_field_id.0));
                    if action_state.just_pressed(UiAction::Chat) {
                        action_state.consume(UiAction::Chat);
                        if input_response.lost_focus() {
                            send_events.send(MessageAccepted);
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

    fn send_message_system(
        mut client_events: EventWriter<ClientMessage>,
        mut chat: ResMut<Chat>,
        local_player: Query<&Name, (With<Authority>, With<Player>)>,
    ) {
        let player_name = local_player.single();
        let message = mem::take(&mut chat.current_message);
        chat.add_player_message(player_name.to_string(), &message);
        client_events.send(ClientMessage::ChatMessage(message));
    }

    fn receive_message_system(
        mut message_events: EventReader<ServerMessage>,
        mut chat: ResMut<Chat>,
    ) {
        for event in message_events.iter() {
            let ServerMessage::ChatMessage { sender_id, message } = event;
            // TODO: Get player name from response
            chat.add_player_message(sender_id.to_string(), message);
        }
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
                    chat.add_message(&format!("Client connected: {}", id));
                }
                ServerEvent::ClientDisconnected(id) => {
                    chat.add_message(&format!("Client disconnected: {}", id));
                }
            }
        }
    }
}

#[derive(Default)]
pub(super) struct Chat {
    text: String,
    active: bool,
    current_message: String,
}

impl Chat {
    fn add_message(&mut self, message: &str) {
        if !self.text.is_empty() {
            self.text.push('\n');
        }
        self.text.push_str(message);
    }

    fn add_player_message(&mut self, player_name: String, message: &str) {
        self.add_message(&format!("[{}]: {}", player_name, message));
    }
}

pub(super) struct InputFieldId(Id);

impl Default for InputFieldId {
    fn default() -> Self {
        Self(Id::new("Input field"))
    }
}

pub(crate) struct MessageAccepted;

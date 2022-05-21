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
    egui::{Align2, Color32, Frame, ScrollArea, TextEdit, TextStyle, Window},
    EguiContext,
};
use bevy_renet::renet::ServerEvent;
use leafwing_input_manager::{plugin::ToggleActions, prelude::ActionState};

use super::{
    back_button::BackButtonPlugin, hud::HudPlugin, ingame_menu::InGameMenuPlugin,
    ui_actions::UiAction, ui_state::UiState, UI_MARGIN,
};
use crate::core::control_actions::ControlAction;

pub(super) struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chat>()
            .add_system(
                Self::chat_system
                    .before(BackButtonPlugin::back_button_system)
                    .before(InGameMenuPlugin::hide_ingame_menu_system)
                    .before(HudPlugin::show_ingame_menu_system),
            )
            .add_system(Self::announce_connected_system)
            .add_system_set(
                SystemSet::on_update(UiState::Hud).with_system(Self::toggle_controls_system),
            );
    }
}

impl ChatPlugin {
    fn chat_system(
        mut input: Local<InputField>,
        mut action_state: ResMut<ActionState<UiAction>>,
        egui: Res<EguiContext>,
        mut chat: ResMut<Chat>,
    ) {
        const CHAT_BOTTOM_MARGIN: f32 = 40.0;

        let frame = if chat.active {
            Frame::window(&egui.ctx().style())
        } else {
            // Show frame with window spacing, but without visuals
            Frame {
                inner_margin: egui.ctx().style().spacing.window_margin,
                rounding: egui.ctx().style().visuals.window_rounding,
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
            .show(egui.ctx(), |ui| {
                if !chat.active {
                    // Hide scrollbar
                    ui.style_mut().visuals.extreme_bg_color = Color32::TRANSPARENT;
                    ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                }

                const CHAT_ROWS_MARGIN: f32 = 4.0;
                const CHAT_VISIBLE_ROWS: usize = 8;
                let text_height = ui.text_style_height(&TextStyle::Body);
                let chat_response = ScrollArea::vertical()
                    .max_height(text_height * CHAT_VISIBLE_ROWS as f32 + CHAT_ROWS_MARGIN)
                    .stick_to_bottom()
                    .enable_scrolling(chat.active)
                    .show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(&mut chat.text.as_str())
                                .desired_rows(CHAT_VISIBLE_ROWS),
                        )
                    })
                    .inner;

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
    }

    fn toggle_controls_system(
        chat: Res<Chat>,
        mut toggle_actions: ResMut<ToggleActions<ControlAction>>,
        mut windows: ResMut<Windows>,
    ) {
        // When controls are enabled and chat is active, we should disable them
        if chat.active == toggle_actions.enabled {
            toggle_actions.enabled = !chat.active;

            let window = windows.get_primary_mut().unwrap();
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

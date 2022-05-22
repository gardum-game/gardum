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

mod controls_settings_tab;
mod video_settings_tab;

use bevy::{
    ecs::system::SystemParam,
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState},
    prelude::*,
};
use bevy_egui::{
    egui::{Align2, Area, Window},
    EguiContext,
};
use leafwing_input_manager::{prelude::ActionState, user_input::InputButton};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::{
    back_button::BackButton, chat::ChatPlugin, ui_actions::UiAction, ui_state::UiState, UI_MARGIN,
};
use crate::core::{
    control_actions::ControlAction,
    game_state::GameState,
    settings::{SettingApplyEvent, Settings},
};
use controls_settings_tab::ControlsSettingsTab;
use video_settings_tab::VideoSettingsTab;

pub(super) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::SettingsMenu)
                .with_system(Self::settings_menu_system)
                .with_system(Self::buttons_system)
                .with_system(Self::binding_window_system.before(ChatPlugin::chat_system))
                .with_system(Self::back_system.after(ChatPlugin::chat_system)),
        );
    }
}

impl SettingsMenuPlugin {
    fn settings_menu_system(
        mut current_tab: Local<SettingsTab>,
        mut commands: Commands,
        windows: Res<Windows>,
        mut egui: ResMut<EguiContext>,
        mut settings: ResMut<Settings>,
    ) {
        let window_width_margin = egui.ctx_mut().style().spacing.window_margin.left * 2.0;

        Window::new("Settings")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(windows.primary().width() - UI_MARGIN * 2.0 - window_width_margin)
            .show(egui.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    for tab in SettingsTab::iter() {
                        ui.selectable_value(&mut *current_tab, tab, tab.to_string());
                    }
                });
                match *current_tab {
                    SettingsTab::Video => VideoSettingsTab::new(&mut settings.video).show(ui),
                    SettingsTab::Control => {
                        ControlsSettingsTab::new(&mut settings.controls).show(ui, &mut commands)
                    }
                };
                ui.expand_to_include_rect(ui.available_rect_before_wrap());
            });
    }

    fn buttons_system(
        mut apply_events: EventWriter<SettingApplyEvent>,
        mut egui: ResMut<EguiContext>,
        mut settings: ResMut<Settings>,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        Area::new("Settings buttons area")
            .anchor(Align2::RIGHT_BOTTOM, (-UI_MARGIN, -UI_MARGIN))
            .show(egui.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Restore defaults").clicked() {
                        *settings = Settings::default();
                        apply_events.send(SettingApplyEvent);
                    }
                    if ui.button("Apply").clicked() {
                        apply_events.send(SettingApplyEvent);
                    }
                    if ui.button("Ok").clicked() {
                        apply_events.send(SettingApplyEvent);
                        action_state.press(UiAction::Back);
                    }
                })
            });
    }

    fn back_system(
        game_state: Res<State<GameState>>,
        mut egui: ResMut<EguiContext>,
        mut action_state: ResMut<ActionState<UiAction>>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        if BackButton::new(&mut action_state)
            .show(egui.ctx_mut())
            .clicked()
        {
            let state = match game_state.current() {
                GameState::Menu => UiState::MainMenu,
                GameState::InGame => UiState::InGameMenu,
            };
            ui_state.set(state).unwrap();
        }
    }

    fn binding_window_system(
        mut commands: Commands,
        mut input_events: InputEvents,
        active_binding: Option<ResMut<ActiveBinding>>,
        mut egui: ResMut<EguiContext>,
        mut settings: ResMut<Settings>,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        let mut active_binding = match active_binding {
            Some(active_binding) => active_binding,
            None => return,
        };

        Window::new(format!("Binding \"{}\"", active_binding.action))
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
                if let Some(conflict) = &active_binding.conflict {
                    ui.label(format!(
                        "Input \"{}\" is already used by \"{}\"",
                        conflict.input_button, conflict.action
                    ));
                    ui.horizontal(|ui| {
                        if ui.button("Replace").clicked() {
                            settings
                                .controls
                                .mappings
                                .remove(conflict.action, conflict.input_button);
                            settings.controls.mappings.insert_at(
                                active_binding.action,
                                conflict.input_button,
                                active_binding.index,
                            );
                            commands.remove_resource::<ActiveBinding>();
                        }
                        if ui.button("Cancel").clicked() {
                            commands.remove_resource::<ActiveBinding>();
                        }
                    });
                } else {
                    ui.label("Press any key now or Esc to cancel");
                    if action_state.just_pressed(UiAction::Back) {
                        action_state.consume(UiAction::Back);
                        commands.remove_resource::<ActiveBinding>();
                    } else if let Some(input_button) = input_events.input_button() {
                        let conflict_action =
                            settings
                                .controls
                                .mappings
                                .iter()
                                .find_map(|(action, inputs)| {
                                    if action != active_binding.action
                                        && inputs.contains(&input_button.into())
                                    {
                                        return Some(action);
                                    }
                                    None
                                });
                        if let Some(action) = conflict_action {
                            active_binding.conflict.replace(BindingConflict {
                                action,
                                input_button,
                            });
                        } else {
                            settings.controls.mappings.insert_at(
                                active_binding.action,
                                input_button,
                                active_binding.index,
                            );
                            commands.remove_resource::<ActiveBinding>();
                        }
                    }
                }
            });
    }
}

struct ActiveBinding {
    action: ControlAction,
    index: usize,
    conflict: Option<BindingConflict>,
}

impl ActiveBinding {
    fn new(action: ControlAction, index: usize) -> Self {
        Self {
            action,
            index,
            conflict: None,
        }
    }
}

struct BindingConflict {
    action: ControlAction,
    input_button: InputButton,
}

#[derive(Display, Clone, Copy, EnumIter, PartialEq)]
enum SettingsTab {
    Video,
    Control,
}

impl Default for SettingsTab {
    fn default() -> Self {
        SettingsTab::Video
    }
}

/// Helper for collecting input
#[derive(SystemParam)]
struct InputEvents<'w, 's> {
    keys: EventReader<'w, 's, KeyboardInput>,
    mouse_buttons: EventReader<'w, 's, MouseButtonInput>,
    gamepad_events: EventReader<'w, 's, GamepadEvent>,
}

impl InputEvents<'_, '_> {
    fn input_button(&mut self) -> Option<InputButton> {
        if let Some(keyboard_input) = self.keys.iter().next() {
            if keyboard_input.state == ElementState::Released {
                if let Some(key_code) = keyboard_input.key_code {
                    return Some(key_code.into());
                }
            }
        }

        if let Some(mouse_input) = self.mouse_buttons.iter().next() {
            if mouse_input.state == ElementState::Released {
                return Some(mouse_input.button.into());
            }
        }

        if let Some(GamepadEvent(_, event_type)) = self.gamepad_events.iter().next() {
            if let GamepadEventType::ButtonChanged(button, strength) = event_type.to_owned() {
                if strength <= 0.5 {
                    return Some(button.into());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, ecs::system::SystemState, input::InputPlugin};

    use super::*;

    #[test]
    fn input_events_reads_keyboard() {
        let mut app = setup_app();
        const KEY: KeyCode = KeyCode::Space;
        let mut keyboard_input = app.world.resource_mut::<Events<KeyboardInput>>();
        keyboard_input.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(KEY),
            state: ElementState::Released,
        });

        let mut system_state: SystemState<InputEvents> = SystemState::new(&mut app.world);
        let mut input_events = system_state.get_mut(&mut app.world);
        let input_button = input_events
            .input_button()
            .expect("Input button should be detected");
        assert_eq!(
            input_button,
            InputButton::Keyboard(KEY),
            "Input button should be equal to the released keyboard key"
        );
    }

    #[test]
    fn input_events_reads_mouse() {
        let mut app = setup_app();
        const BUTTON: MouseButton = MouseButton::Right;
        let mut mouse_button = app.world.resource_mut::<Events<MouseButtonInput>>();
        mouse_button.send(MouseButtonInput {
            button: BUTTON,
            state: ElementState::Released,
        });

        let mut system_state: SystemState<InputEvents> = SystemState::new(&mut app.world);
        let mut input_events = system_state.get_mut(&mut app.world);
        let input_button = input_events
            .input_button()
            .expect("Input button should be detected");
        assert_eq!(
            input_button,
            InputButton::Mouse(BUTTON),
            "Input button should be equal to the released mouse button"
        );
    }

    #[test]
    fn input_events_reads_gamepad() {
        let mut app = setup_app();
        const BUTTON: GamepadButtonType = GamepadButtonType::Z;
        const PRESSED_STRENGTH: f32 = 0.6;
        let mut gamepad_events = app.world.resource_mut::<Events<GamepadEvent>>();
        gamepad_events.send(GamepadEvent(
            Gamepad(0),
            GamepadEventType::ButtonChanged(BUTTON, PRESSED_STRENGTH),
        ));

        let mut system_state: SystemState<InputEvents> = SystemState::new(&mut app.world);
        let mut input_events = system_state.get_mut(&mut app.world);
        assert_eq!(
            input_events.input_button(),
            None,
            "Input button shouldn't be detected when pressed strength is {PRESSED_STRENGTH}"
        );

        const RELEASED_STRENGTH: f32 = 0.5;
        let mut gamepad_events = app.world.resource_mut::<Events<GamepadEvent>>();
        gamepad_events.send(GamepadEvent(
            Gamepad(0),
            GamepadEventType::ButtonChanged(BUTTON, RELEASED_STRENGTH),
        ));

        let mut input_events = system_state.get_mut(&mut app.world);
        let input_button = input_events
            .input_button()
            .expect("Input button should be detected with {RELEASED_STRENGTH} strength");
        assert_eq!(
            input_button,
            InputButton::Gamepad(BUTTON),
            "Input button should be equal to the released gamepad button"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(InputPlugin);
        app
    }
}

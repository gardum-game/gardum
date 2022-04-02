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

use bevy::{
    ecs::system::SystemParam,
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState},
    prelude::*,
};
use bevy_egui::{
    egui::{Align2, Area, ComboBox, Grid, Ui, Window},
    EguiContext,
};
use leafwing_input_manager::{
    buttonlike_user_input::InputButton,
    prelude::{ActionState, UserInput},
    Actionlike,
};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::{
    back_button::BackButtonsSystems,
    ui_action::UiAction,
    ui_state::{UiState, UiStateHistory},
    UI_MARGIN,
};
use crate::core::{
    settings::CharacterAction,
    settings::{ControlSettings, SettingApplyEvent, Settings, VideoSettings},
};

pub(super) struct SettingMenuPlugin;

impl Plugin for SettingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::SettingsMenu)
                .with_system(settings_menu_system)
                .with_system(settings_buttons_system),
        )
        .add_system_set(
            SystemSet::on_update(UiState::SettingsMenu)
                .with_system(show_binding_window_system)
                .before(BackButtonsSystems::BackButton),
        );
    }
}

fn settings_menu_system(
    mut commands: Commands,
    egui: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut settings: ResMut<Settings>,
    mut current_tab: Local<SettingsTab>,
) {
    let main_window = windows.get_primary().unwrap();
    let window_width_margin = egui.ctx().style().spacing.window_margin.left * 2.0;

    Window::new("Settings")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .default_width(main_window.width() - UI_MARGIN * 2.0 - window_width_margin)
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                for tab in SettingsTab::iter() {
                    ui.selectable_value(&mut *current_tab, tab, tab.to_string());
                }
            });
            match *current_tab {
                SettingsTab::Video => show_video_settings(ui, &mut settings.video),
                SettingsTab::Control => show_control_settings(
                    &mut commands,
                    ui,
                    window_width_margin,
                    &mut settings.control,
                ),
            };
            ui.expand_to_include_rect(ui.available_rect_before_wrap());
        });
}

fn settings_buttons_system(
    egui: ResMut<EguiContext>,
    mut apply_events: EventWriter<SettingApplyEvent>,
    mut settings: ResMut<Settings>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    Area::new("Settings buttons area")
        .anchor(Align2::RIGHT_BOTTOM, (-UI_MARGIN, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
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
                    ui_state_history.pop();
                }
            })
        });
}

fn show_video_settings(ui: &mut Ui, video_settings: &mut VideoSettings) {
    ComboBox::from_label("MSAA samples")
        .selected_text(video_settings.msaa.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut video_settings.msaa, 1, 1.to_string());
            ui.selectable_value(&mut video_settings.msaa, 4, 4.to_string());
        });
    ui.checkbox(
        &mut video_settings.global_illumination,
        "Global illumination",
    );
}

fn show_control_settings(
    commands: &mut Commands,
    ui: &mut Ui,
    window_width_margin: f32,
    control_settings: &mut ControlSettings,
) {
    const INPUT_VARIANTS: usize = 3;
    const COLUMNS_COUNT: usize = INPUT_VARIANTS + 1;

    Grid::new("Control grid")
        .num_columns(COLUMNS_COUNT)
        .striped(true)
        .min_col_width(ui.available_width() / COLUMNS_COUNT as f32 - window_width_margin)
        .show(ui, |ui| {
            for action in CharacterAction::variants() {
                ui.label(action.to_string());
                let inputs = control_settings.mappings.get(action);
                for index in 0..INPUT_VARIANTS {
                    let button_text = match inputs.get_at(index) {
                        Some(UserInput::Single(InputButton::Gamepad(gamepad_button))) => {
                            format!("🎮 {:?}", gamepad_button)
                        }
                        Some(UserInput::Single(InputButton::Keyboard(keycode))) => {
                            format!("🖮 {:?}", keycode)
                        }
                        Some(UserInput::Single(InputButton::Mouse(mouse_button))) => {
                            format!("🖱 {:?}", mouse_button)
                        }
                        _ => "Empty".to_string(),
                    };
                    if ui.button(button_text).clicked() {
                        commands.insert_resource(ActiveBinding::new(action, index));
                    }
                }
                ui.end_row();
            }
        });
}

fn show_binding_window_system(
    mut commands: Commands,
    egui: ResMut<EguiContext>,
    mut input_events: InputEvents,
    active_binding: Option<ResMut<ActiveBinding>>,
    mut settings: ResMut<Settings>,
    mut ui_actions: Query<&mut ActionState<UiAction>>,
) {
    let mut active_binding = match active_binding {
        Some(active_binding) => active_binding,
        None => return,
    };

    Window::new(format!("Binding \"{}\"", active_binding.action))
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            if let Some(conflict) = &active_binding.conflict {
                ui.label(format!(
                    "Input {} is already used by {}",
                    conflict.input_button, conflict.action
                ));
                ui.horizontal(|ui| {
                    if ui.button("Replace").clicked() {
                        settings
                            .control
                            .mappings
                            .remove(conflict.action, conflict.input_button);
                        settings.control.mappings.insert_at(
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
                let mut ui_actions = ui_actions.single_mut();
                if ui_actions.just_pressed(UiAction::Back) {
                    ui_actions.make_held(UiAction::Back);
                    commands.remove_resource::<ActiveBinding>();
                } else if let Some(input_button) = input_events.input_button() {
                    let conflict_action = settings.control.mappings.iter().enumerate().find_map(
                        |(action, inputs)| {
                            let action = CharacterAction::get_at(action).unwrap();
                            if action != active_binding.action
                                && inputs.contains(&input_button.into())
                            {
                                return Some(action);
                            }
                            None
                        },
                    );
                    if let Some(action) = conflict_action {
                        active_binding.conflict.replace(BindingConflict {
                            action,
                            input_button,
                        });
                    } else {
                        settings.control.mappings.insert_at(
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

struct ActiveBinding {
    action: CharacterAction,
    index: usize,
    conflict: Option<BindingConflict>,
}

impl ActiveBinding {
    fn new(action: CharacterAction, index: usize) -> Self {
        Self {
            action,
            index,
            conflict: None,
        }
    }
}

struct BindingConflict {
    action: CharacterAction,
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

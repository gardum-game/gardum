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

mod controls_tab;
#[cfg(feature = "developer")]
mod developer_tab;
mod input_events;
mod video_tab;

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Area, Window},
    EguiContext,
};
use iyes_loopless::prelude::*;
use leafwing_input_manager::{prelude::ActionState, user_input::InputButton};
use strum::{Display, EnumIter, IntoEnumIterator};

use super::{
    back_button::BackButton, chat_window::ChatWindowPlugin, modal_window::ModalWindow,
    ui_actions::UiAction, ui_state::UiState, UI_MARGIN,
};
use crate::core::{
    control_actions::ControlAction,
    game_state::GameState,
    settings::{Settings, SettingsApplied},
};
use controls_tab::ControlsTab;
#[cfg(feature = "developer")]
use developer_tab::DeveloperTab;
use input_events::InputEvents;
use video_tab::VideoTab;

pub(super) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::settings_menu_system.run_in_state(UiState::SettingsMenu))
            .add_system(Self::buttons_system.run_in_state(UiState::SettingsMenu))
            .add_system(
                Self::binding_window_system
                    .run_in_state(UiState::SettingsMenu)
                    .run_if_resource_exists::<ActiveBinding>()
                    .before(ChatWindowPlugin::chat_system),
            )
            .add_system(
                Self::back_system
                    .run_in_state(UiState::SettingsMenu)
                    .after(ChatWindowPlugin::chat_system),
            );
    }
}

impl SettingsMenuPlugin {
    fn settings_menu_system(
        mut current_tab: Local<SettingsTab>,
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        mut settings: ResMut<Settings>,
    ) {
        let window_width_margin = egui.ctx_mut().style().spacing.window_margin.left * 2.0;
        let screen_rect = egui.ctx_mut().input().screen_rect();

        Window::new("Settings")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(screen_rect.width() - UI_MARGIN * 2.0 - window_width_margin)
            .show(egui.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    for tab in SettingsTab::iter() {
                        ui.selectable_value(&mut *current_tab, tab, tab.to_string());
                    }
                });
                match *current_tab {
                    SettingsTab::Video => VideoTab::new(&mut settings.video).show(ui),
                    SettingsTab::Control => {
                        ControlsTab::new(&mut settings.controls).show(ui, &mut commands)
                    }
                    #[cfg(feature = "developer")]
                    SettingsTab::Developer => DeveloperTab::new(&mut settings.developer).show(ui),
                };
                ui.expand_to_include_rect(ui.available_rect_before_wrap());
            });
    }

    fn buttons_system(
        mut apply_events: EventWriter<SettingsApplied>,
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
                        apply_events.send(SettingsApplied);
                    }
                    if ui.button("Apply").clicked() {
                        apply_events.send(SettingsApplied);
                    }
                    if ui.button("Ok").clicked() {
                        apply_events.send(SettingsApplied);
                        action_state.press(UiAction::Back);
                    }
                })
            });
    }

    fn back_system(
        mut commands: Commands,
        game_state: Res<State<GameState>>,
        mut egui: ResMut<EguiContext>,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        if BackButton::new(&mut action_state)
            .show(egui.ctx_mut())
            .clicked()
        {
            let state = match game_state.current() {
                GameState::Menu => UiState::MainMenu,
                GameState::InGame => UiState::InGameMenu,
            };
            commands.insert_resource(NextState(state));
        }
    }

    fn binding_window_system(
        mut commands: Commands,
        mut input_events: InputEvents,
        mut active_binding: ResMut<ActiveBinding>,
        mut egui: ResMut<EguiContext>,
        mut settings: ResMut<Settings>,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        ModalWindow::new(format!("Binding \"{}\"", active_binding.action)).show(
            egui.ctx_mut(),
            |ui| {
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
            },
        );
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
    #[cfg(feature = "developer")]
    Developer,
}

impl Default for SettingsTab {
    fn default() -> Self {
        SettingsTab::Video
    }
}

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

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};
use leafwing_input_manager::prelude::ActionState;

use super::{
    ui_state::{UiState, UiStateHistory},
    UiAction,
};

#[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub(super) enum InGameMenuSystems {
    HideInGameMenu,
    ShowInGameMenu,
}

pub(super) struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::InGameMenu)
                .with_system(ingame_menu_system)
                .with_system(hide_ingame_menu_system.label(InGameMenuSystems::HideInGameMenu)),
        )
        .add_system_set(
            SystemSet::on_update(UiState::Hud)
                .with_system(show_ingame_menu_system.label(InGameMenuSystems::ShowInGameMenu)),
        );
    }
}

fn ingame_menu_system(
    egui: ResMut<EguiContext>,
    mut exit_event: EventWriter<AppExit>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    Area::new("Main Menu")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .show(egui.ctx(), |ui| {
            if ui.button("Resume").clicked() {
                ui_state_history.pop();
            }
            if ui.button("Settings").clicked() {
                ui_state_history.push(UiState::SettingsMenu);
            }
            if ui.button("Main menu").clicked() {
                ui_state_history.clear();
                ui_state_history.push(UiState::MainMenu);
            }
            if ui.button("Exit").clicked() {
                exit_event.send(AppExit);
            }
        });
}

fn show_ingame_menu_system(
    mut action_state: ResMut<ActionState<UiAction>>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    if action_state.just_pressed(UiAction::Back) {
        action_state.consume(UiAction::Back);
        ui_state_history.push(UiState::InGameMenu);
    }
}

fn hide_ingame_menu_system(
    mut action_state: ResMut<ActionState<UiAction>>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    if action_state.just_pressed(UiAction::Back) {
        action_state.consume(UiAction::Back);
        ui_state_history.pop();
    }
}

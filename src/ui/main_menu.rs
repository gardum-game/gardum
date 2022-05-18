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
    egui::{Align2, Area, Button, RichText, TextStyle},
    EguiContext,
};

use super::{ui_state::UiState, UI_MARGIN};
use crate::core::game_state::GameState;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::MainMenu).with_system(Self::main_menu_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Menu).with_system(Self::show_main_menu_system),
        );
    }
}

impl MainMenuPlugin {
    fn main_menu_system(
        egui: ResMut<EguiContext>,
        mut exit_event: EventWriter<AppExit>,
        mut ui_state: ResMut<State<UiState>>,
    ) {
        Area::new("Main Menu")
            .anchor(Align2::LEFT_CENTER, (UI_MARGIN, 0.0))
            .show(egui.ctx(), |ui| {
                ui.add_enabled(
                    false,
                    Button::new(RichText::new("Play").text_style(TextStyle::Heading)),
                );
                if ui
                    .add(Button::new(
                        RichText::new("Custom game").text_style(TextStyle::Heading),
                    ))
                    .clicked()
                {
                    ui_state.set(UiState::ServerBrowser).unwrap();
                }
                ui.add_enabled(
                    false,
                    Button::new(RichText::new("Characters").text_style(TextStyle::Heading)),
                );
                if ui
                    .add(Button::new(
                        RichText::new("Settings").text_style(TextStyle::Heading),
                    ))
                    .clicked()
                {
                    ui_state.set(UiState::SettingsMenu).unwrap();
                }
                if ui
                    .add(Button::new(
                        RichText::new("Exit").text_style(TextStyle::Heading),
                    ))
                    .clicked()
                {
                    exit_event.send(AppExit);
                }
            });
    }

    fn show_main_menu_system(mut ui_state: ResMut<State<UiState>>) {
        ui_state.set(UiState::MainMenu).unwrap();
    }
}

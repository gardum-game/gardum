/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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
    egui::{Align2, Area, Button, DragValue, Grid, TextStyle, Window},
    EguiContext,
};

use crate::core::{AppState, GameSettings};

const MARGIN: f32 = 20.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MainMenuState {
    Disabled,
    Idle,
    CustomGame,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SearchText>()
            .add_state(MainMenuState::Idle)
            .add_system_set(
                SystemSet::on_update(MainMenuState::Idle).with_system(main_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(MainMenuState::CustomGame)
                    .with_system(custom_game_window_system.system())
                    .with_system(back_button_system.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(disable_main_menu_system.system()),
            );
    }
}

fn main_menu_system(
    egui: ResMut<EguiContext>,
    mut exit_event: EventWriter<AppExit>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
) {
    Area::new("Main Menu")
        .anchor(Align2::LEFT_CENTER, (MARGIN, 0.0))
        .show(egui.ctx(), |ui| {
            ui.add_enabled(false, Button::new("Play").text_style(TextStyle::Heading));
            if ui
                .add(Button::new("Custom game").text_style(TextStyle::Heading))
                .clicked()
            {
                main_menu_state.set(MainMenuState::CustomGame).unwrap();
            }
            ui.add_enabled(
                false,
                Button::new("Characters").text_style(TextStyle::Heading),
            );
            ui.add_enabled(
                false,
                Button::new("Settings").text_style(TextStyle::Heading),
            );
            if ui
                .add(Button::new("Exit").text_style(TextStyle::Heading))
                .clicked()
            {
                exit_event.send(AppExit);
            }
        });
}

#[derive(Default)]
struct SearchText(String);

fn custom_game_window_system(
    egui: ResMut<EguiContext>,
    mut game_settings: ResMut<GameSettings>,
    mut search_text: Local<SearchText>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Custom game")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut search_text.0);
                if ui.button("Create").clicked() || ui.button("Connect").clicked() {
                    app_state.set(AppState::InGame).unwrap();
                }
                ui.group(|ui| {
                    let mut teams_enabled = game_settings.teams_count.is_some();
                    let mut teams_count = game_settings.teams_count.unwrap_or(0);
                    Grid::new("Server Settings").show(ui, |ui| {
                        ui.label("Map:");
                        ui.text_edit_singleline(&mut game_settings.map);
                        ui.end_row();
                        ui.checkbox(&mut teams_enabled, "Teams enabled");
                        ui.end_row();
                        ui.label("Teams count:");
                        ui.add_enabled(teams_enabled, DragValue::new(&mut teams_count));
                        ui.end_row();
                        ui.label("Slots count:");
                        ui.add(DragValue::new(&mut game_settings.slots_count));
                    });
                    game_settings.teams_count = if teams_enabled {
                        Some(teams_count)
                    } else {
                        None
                    };
                });
            })
        });
}

fn back_button_system(
    egui: ResMut<EguiContext>,
    input: Res<Input<KeyCode>>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
) {
    Area::new("Back area")
        .anchor(Align2::LEFT_BOTTOM, (MARGIN, -MARGIN))
        .show(egui.ctx(), |ui| {
            if input.just_pressed(KeyCode::Escape) || ui.button("Back").clicked() {
                main_menu_state.set(MainMenuState::Idle).unwrap();
            }
        });
}

fn disable_main_menu_system(mut main_menu_state: ResMut<State<MainMenuState>>) {
    main_menu_state.set(MainMenuState::Disabled).unwrap();
}

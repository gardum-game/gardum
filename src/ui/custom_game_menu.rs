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

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, ComboBox, DragValue, Grid, SidePanel, TextEdit, Ui, Window},
    EguiContext,
};
use strum::IntoEnumIterator;

use crate::core::{gamemodes::GameMode, AppState, GameSettings};

pub struct CustomGameMenuPlugin;

impl Plugin for CustomGameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SearchText>()
            .add_system_set(
                SystemSet::on_update(AppState::CustomGameMenu)
                    .with_system(custom_game_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::DirectConnectMenu)
                    .with_system(direct_connect_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::CreateGameMenu)
                    .with_system(create_game_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::LobbyMenu).with_system(lobby_menu_system.system()),
            );
    }
}

#[derive(Default)]
struct SearchText(String);

fn custom_game_menu_system(
    egui: ResMut<EguiContext>,
    mut search_text: Local<SearchText>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Custom game")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled(
                    false,
                    TextEdit::singleline(&mut search_text.0).hint_text("Search servers"),
                );
                if ui.button("Connect").clicked() {
                    app_state.push(AppState::DirectConnectMenu).unwrap();
                }
                if ui.button("Create").clicked() {
                    app_state.push(AppState::CreateGameMenu).unwrap();
                }
            });
            ui.add_space(400.0);
        });
}

fn create_game_menu_system(
    egui: ResMut<EguiContext>,
    mut game_settings: ResMut<GameSettings>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Custom game")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.vertical(|ui| {
                show_server_settings(ui, &mut game_settings);
                if ui.button("Create").clicked() {
                    app_state.push(AppState::LobbyMenu).unwrap();
                }
            })
        });
}

fn lobby_menu_system(
    egui: ResMut<EguiContext>,
    mut game_settings: ResMut<GameSettings>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Lobby")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    show_teams(ui, game_settings.game_mode);
                    SidePanel::right("Server settings").show_inside(ui, |ui| {
                        show_server_settings(ui, &mut game_settings);
                    })
                });
                if ui.button("Start").clicked() {
                    app_state.replace(AppState::InGame).unwrap();
                }
            })
        });
}

fn show_server_settings(ui: &mut Ui, game_settings: &mut GameSettings) {
    Grid::new("Server settings grid").show(ui, |ui| {
        ui.heading("Settings");
        ui.end_row();
        ui.label("Game name:");
        ui.text_edit_singleline(&mut game_settings.game_name);
        ui.end_row();
        ui.label("Port:");
        ui.add(DragValue::new(&mut game_settings.port));
        ui.end_row();
        ui.label("GameMode:");
        ComboBox::from_id_source("Game mode")
            .selected_text(format!("{:?}", game_settings.game_mode))
            .show_ui(ui, |ui| {
                for game_mode in GameMode::iter() {
                    ui.selectable_value(
                        &mut game_settings.game_mode,
                        game_mode,
                        format!("{:?}", game_mode),
                    );
                }
            });
    });
}

fn show_teams(ui: &mut Ui, game_mode: GameMode) {
    ui.vertical(|ui| {
        ui.heading("Players");
        for _ in 0..game_mode.slots_count() {
            ui.label("Empty slot");
        }
    });
}

struct DirectConnectData {
    ip: String,
    port: String,
}

impl Default for DirectConnectData {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: "4761".to_string(),
        }
    }
}

fn direct_connect_menu_system(
    egui: ResMut<EguiContext>,
    mut data: Local<DirectConnectData>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Direct connect")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            Grid::new("Direct connect grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("IP:");
                    ui.text_edit_singleline(&mut data.ip);
                    ui.end_row();
                    ui.label("Port:");
                    ui.text_edit_singleline(&mut data.port);
                    ui.end_row();
                });
            ui.vertical_centered(|ui| {
                if ui.button("Connect").clicked() {
                    app_state.replace(AppState::InGame).unwrap();
                }
            });
        });
}

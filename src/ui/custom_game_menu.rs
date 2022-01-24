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
    egui::{Align2, ComboBox, DragValue, Grid, SidePanel, TextEdit, Ui, Window},
    EguiContext,
};
use strum::IntoEnumIterator;

use super::ui_state::{UiState, UiStateHistory};
use crate::{
    core::{player::Nickname, AppState, ServerSettings},
    game_modes::GameMode,
    maps::Map,
};

pub(super) struct CustomGameMenuPlugin;

impl Plugin for CustomGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SearchText>()
            .add_system_set(
                SystemSet::on_update(UiState::CustomGameMenu).with_system(custom_game_menu_system),
            )
            .add_system_set(
                SystemSet::on_update(UiState::DirectConnectMenu)
                    .with_system(direct_connect_menu_system),
            )
            .add_system_set(
                SystemSet::on_update(UiState::CreateGameMenu).with_system(create_game_menu_system),
            )
            .add_system_set(SystemSet::on_update(UiState::LobbyMenu).with_system(lobby_menu_system))
            .add_system_set(
                SystemSet::on_enter(AppState::Lobby).with_system(show_lobby_menu_system),
            );
    }
}

#[derive(Default)]
struct SearchText(String);

fn custom_game_menu_system(
    egui: ResMut<EguiContext>,
    mut search_text: Local<SearchText>,
    mut ui_state_history: ResMut<UiStateHistory>,
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
                    ui_state_history.push(UiState::DirectConnectMenu);
                }
                if ui.button("Create").clicked() {
                    ui_state_history.push(UiState::CreateGameMenu);
                }
            });
            ui.add_space(400.0);
        });
}

fn create_game_menu_system(
    egui: ResMut<EguiContext>,
    mut server_settings: ResMut<ServerSettings>,
    mut game_mode: ResMut<GameMode>,
    mut map: ResMut<Map>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Custom game")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.vertical(|ui| {
                show_game_settings(ui, &mut server_settings, &mut game_mode, &mut map);
                if ui.button("Create").clicked() {
                    app_state.set(AppState::Lobby).unwrap();
                }
            })
        });
}

fn lobby_menu_system(
    egui: ResMut<EguiContext>,
    nicknames_query: Query<&Nickname>,
    mut server_settings: ResMut<ServerSettings>,
    mut game_mode: ResMut<GameMode>,
    mut map: ResMut<Map>,
    mut app_state: ResMut<State<AppState>>,
) {
    Window::new("Lobby")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    show_teams(ui, *game_mode, nicknames_query.iter().collect());
                    SidePanel::right("Server settings").show_inside(ui, |ui| {
                        show_game_settings(ui, &mut server_settings, &mut game_mode, &mut map);
                    })
                });
                if ui.button("Start").clicked() {
                    app_state.set(AppState::InGame).unwrap();
                }
            })
        });
}

fn show_game_settings(
    ui: &mut Ui,
    server_settings: &mut ServerSettings,
    current_game_mode: &mut GameMode,
    current_map: &mut Map,
) {
    Grid::new("Server settings grid").show(ui, |ui| {
        ui.heading("Settings");
        ui.end_row();
        ui.label("Game name:");
        ui.text_edit_singleline(&mut server_settings.game_name);
        ui.end_row();
        ui.label("Port:");
        ui.add(DragValue::new(&mut server_settings.port));
        ui.end_row();
        ui.label("Game mode:");
        ComboBox::from_id_source("Game mode")
            .selected_text(format!("{:?}", current_game_mode))
            .show_ui(ui, |ui| {
                for mode in GameMode::iter() {
                    ui.selectable_value(current_game_mode, mode, format!("{:?}", mode));
                }
            });
        ui.end_row();
        ui.label("Map:");
        ComboBox::from_id_source("Map")
            .selected_text(format!("{:?}", current_map))
            .show_ui(ui, |ui| {
                for map in Map::iter() {
                    ui.selectable_value(current_map, map, format!("{:?}", map));
                }
            });
    });
}

fn show_teams(ui: &mut Ui, current_game_mode: GameMode, nicknames: Vec<&Nickname>) {
    ui.vertical(|ui| {
        ui.heading("Players");
        for i in 0..current_game_mode.slots_count() {
            if let Some(nickname) = nicknames.get(i as usize) {
                ui.label(nickname.0.clone());
            } else {
                ui.label("Empty slot");
            }
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
                    app_state.set(AppState::InGame).unwrap();
                }
            });
        });
}

fn show_lobby_menu_system(mut ui_state_history: ResMut<UiStateHistory>) {
    ui_state_history.push(UiState::LobbyMenu);
}
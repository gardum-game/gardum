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
    egui::{Align2, ComboBox, DragValue, Grid, SidePanel, Ui, Window},
    EguiContext,
};
use strum::IntoEnumIterator;

use crate::{
    core::{game_state::GameState, map::Map, server_settings::ServerSettings, session::GameMode},
    ui::ui_state::{UiState, UiStateHistory},
};

pub(super) struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::CrateLobbyMenu).with_system(create_lobby_menu_system),
        )
        .add_system_set(SystemSet::on_update(UiState::LobbyMenu).with_system(lobby_menu_system))
        .add_system_set(SystemSet::on_enter(GameState::Lobby).with_system(show_lobby_menu_system));
    }
}

fn create_lobby_menu_system(
    egui: ResMut<EguiContext>,
    mut server_settings: ResMut<ServerSettings>,
    mut game_mode: ResMut<GameMode>,
    mut map: ResMut<Map>,
    mut game_state: ResMut<State<GameState>>,
) {
    Window::new("Create lobby")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.vertical(|ui| {
                show_game_settings(ui, &mut server_settings, &mut game_mode, &mut map);
                if ui.button("Create").clicked() {
                    game_state.set(GameState::Lobby).unwrap();
                }
            })
        });
}

fn lobby_menu_system(
    egui: ResMut<EguiContext>,
    names: Query<&Name>,
    mut server_settings: ResMut<ServerSettings>,
    mut game_mode: ResMut<GameMode>,
    mut map: ResMut<Map>,
    mut game_state: ResMut<State<GameState>>,
) {
    Window::new("Lobby")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    show_teams(ui, *game_mode, names.iter().collect());
                    SidePanel::right("Server settings").show_inside(ui, |ui| {
                        show_game_settings(ui, &mut server_settings, &mut game_mode, &mut map);
                    })
                });
                if ui.button("Start").clicked() {
                    game_state.set(GameState::InGame).unwrap();
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

fn show_teams(ui: &mut Ui, current_game_mode: GameMode, names: Vec<&Name>) {
    ui.vertical(|ui| {
        ui.heading("Players");
        for i in 0..current_game_mode.slots_count() {
            if let Some(name) = names.get(i as usize) {
                ui.label(name.as_str());
            } else {
                ui.label("Empty slot");
            }
        }
    });
}

fn show_lobby_menu_system(mut ui_state_history: ResMut<UiStateHistory>) {
    ui_state_history.push(UiState::LobbyMenu);
}
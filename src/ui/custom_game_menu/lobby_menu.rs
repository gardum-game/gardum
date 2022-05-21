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
    egui::{Align2, Button, ComboBox, DragValue, Grid, Ui, Window},
    EguiContext,
};
use bevy_renet::renet::{RenetClient, RenetServer};
use strum::IntoEnumIterator;

use crate::{
    core::{
        game_state::GameState, map::Map, network::server::ServerSettings, player::Player,
        session::GameMode,
    },
    ui::{error_dialog::ErrorDialog, ui_state::UiState},
};

pub(super) struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::LobbyMenu).with_system(Self::lobby_menu_system),
        );
    }
}

#[allow(clippy::too_many_arguments)]
impl LobbyMenuPlugin {
    fn lobby_menu_system(
        mut commands: Commands,
        client: Option<Res<RenetClient>>,
        server: Option<Res<RenetServer>>,
        mut egui: ResMut<EguiContext>,
        mut server_settings: ResMut<ServerSettings>,
        mut game_state: ResMut<State<GameState>>,
        mut ui_state: ResMut<State<UiState>>,
        mut error_dialog: ResMut<ErrorDialog>,
        player_names: Query<&Name, With<Player>>,
    ) {
        Window::new("Lobby")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
                ui.horizontal_top(|ui| {
                    if client.is_some() || server.is_some() {
                        Self::show_teams(
                            ui,
                            server_settings.game_mode.slots_count(),
                            player_names.iter().collect(),
                        );
                    }
                    Self::show_game_settings(ui, &mut server_settings);
                });
                ui.vertical_centered(|ui| {
                    if client.is_none() && server.is_none() {
                        if ui.button("Create").clicked() {
                            match server_settings.create_server() {
                                Ok(server) => commands.insert_resource(server),
                                Err(error) => error_dialog.show(
                                    "Unable to create server".to_string(),
                                    error.to_string(),
                                    &mut ui_state,
                                ),
                            }
                        }
                    } else if ui
                        .add_enabled(server.is_some(), Button::new("Start"))
                        .clicked()
                    {
                        game_state.set(GameState::InGame).unwrap();
                    }
                })
            });
    }

    fn show_game_settings(ui: &mut Ui, server_settings: &mut ServerSettings) {
        Grid::new("Server settings grid").show(ui, |ui| {
            ui.heading("Settings");
            ui.end_row();
            ui.label("Server name:");
            ui.text_edit_singleline(&mut server_settings.server_name);
            ui.end_row();
            ui.label("Port:");
            ui.add(DragValue::new(&mut server_settings.port));
            ui.end_row();
            ui.label("Game mode:");
            ComboBox::from_id_source("Game mode")
                .selected_text(format!("{:?}", &mut server_settings.game_mode))
                .show_ui(ui, |ui| {
                    for game_mode in GameMode::iter() {
                        ui.selectable_value(
                            &mut server_settings.game_mode,
                            game_mode,
                            format!("{:?}", game_mode),
                        );
                    }
                });
            ui.end_row();
            ui.label("Map:");
            ComboBox::from_id_source("Map")
                .selected_text(format!("{:?}", &mut server_settings.map))
                .show_ui(ui, |ui| {
                    for map in Map::iter() {
                        ui.selectable_value(&mut server_settings.map, map, format!("{:?}", map));
                    }
                });
            ui.end_row();
            ui.checkbox(&mut server_settings.random_heroes, "Random heroes:");
            ui.end_row();
        });
    }

    fn show_teams(ui: &mut Ui, slots_count: u8, names: Vec<&Name>) {
        ui.vertical(|ui| {
            ui.heading("Players");
            for i in 0..slots_count {
                if let Some(name) = names.get(i as usize) {
                    ui.label(name.as_str());
                } else {
                    ui.label("Empty slot");
                }
            }
        });
    }
}

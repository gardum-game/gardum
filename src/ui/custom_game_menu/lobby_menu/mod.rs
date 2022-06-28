/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

mod players_grid;
mod server_settings_grid;

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Button, Window},
    EguiContext,
};
use bevy_renet::renet::{RenetClient, RenetServer};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    core::{game_state::GameState, network::server::ServerSettings, player::Player},
    ui::{
        back_button::BackButton, error_dialog::ErrorMessage, modal_window::ModalWindow,
        ui_actions::UiAction, ui_state::UiState,
    },
};
use players_grid::PlayersGrid;
use server_settings_grid::GameSettingsGrid;

pub(super) struct LobbyMenuPlugin;

impl Plugin for LobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::lobby_menu_system.run_in_state(UiState::LobbyMenu))
            .add_system(Self::back_system.run_in_state(UiState::LobbyMenu))
            .add_system(
                Self::confirmation_dialog_system
                    .run_if_resource_exists::<Confirmation>()
                    .run_in_state(UiState::LobbyMenu),
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
        player_names: Query<&Name, With<Player>>,
    ) {
        Window::new("Lobby")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
                ui.horizontal_top(|ui| {
                    if client.is_some() || server.is_some() {
                        PlayersGrid::new(
                            player_names.iter(),
                            server_settings.game_mode.slots_count(),
                        )
                        .show(ui);
                    }
                    GameSettingsGrid::new(&mut server_settings, server.is_some()).show(ui);
                });
                ui.vertical_centered(|ui| {
                    if client.is_none() && server.is_none() {
                        if ui.button("Create").clicked() {
                            match server_settings.create_server() {
                                Ok(server) => commands.insert_resource(server),
                                Err(error) => commands.insert_resource(ErrorMessage {
                                    title: "Unable to create server".to_string(),
                                    text: error.to_string(),
                                }),
                            }
                        }
                    } else if ui
                        .add_enabled(server.is_some(), Button::new("Start"))
                        .clicked()
                    {
                        commands.insert_resource(NextState(GameState::InGame));
                    }
                })
            });
    }

    fn back_system(
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        if BackButton::new(&mut action_state)
            .show(egui.ctx_mut())
            .clicked()
        {
            commands.init_resource::<Confirmation>();
        }
    }

    fn confirmation_dialog_system(mut commands: Commands, mut egui: ResMut<EguiContext>) {
        ModalWindow::new("Exit lobby").show(egui.ctx_mut(), |ui| {
            ui.label("Are you shoule you want to leave?");
            ui.horizontal(|ui| {
                if ui.button("Yes").clicked() {
                    commands.insert_resource(NextState(UiState::ServerBrowser));
                    commands.remove_resource::<Confirmation>();
                }
                if ui.button("No").clicked() {
                    commands.remove_resource::<Confirmation>();
                }
            })
        });
    }
}

#[derive(Default)]
struct Confirmation;

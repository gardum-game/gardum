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
    egui::{Align2, Grid, Window},
    EguiContext,
};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use super::{ui_actions::UiAction, ui_state::UiState};
use crate::core::player::{Damage, Deaths, Healing, Kills};

pub(super) struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            Self::scoreboard_system
                .run_in_state(UiState::Hud)
                .run_if(Self::scoreboard_action_pressed),
        );
    }
}

impl ScoreboardPlugin {
    fn scoreboard_system(
        mut egui: ResMut<EguiContext>,
        players: Query<(&Name, &Kills, &Deaths, &Damage, &Healing)>,
    ) {
        Window::new("Scoreboard")
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
                Grid::new("Scoreboard grid").striped(true).show(ui, |ui| {
                    ui.label("Player");
                    ui.label("Kills");
                    ui.label("Deaths");
                    ui.label("Damage");
                    ui.label("Healing");
                    ui.end_row();

                    for (name, kills, deaths, damage, healing) in players.iter() {
                        ui.label(name.as_str());
                        ui.label(kills.to_string());
                        ui.label(deaths.to_string());
                        ui.label(damage.to_string());
                        ui.label(healing.to_string());
                        ui.end_row();
                    }
                })
            });
    }

    fn scoreboard_action_pressed(action_state: Res<ActionState<UiAction>>) -> bool {
        action_state.pressed(UiAction::Scoreboard)
    }
}

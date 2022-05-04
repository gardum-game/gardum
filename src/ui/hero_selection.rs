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
    egui::{epaint::WHITE_UV, vec2, Align2, Area, ImageButton, Rect, TextureId, Window},
    EguiContext,
};
use strum::IntoEnumIterator;

use super::{
    ui_state::{UiState, UiStateHistory},
    UI_MARGIN,
};
use crate::core::{
    character::hero::HeroKind, game_state::GameState, network::server::ServerSettings,
    player::Player, Authority,
};

pub struct HeroSelectionPlugin;

impl Plugin for HeroSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::HeroSelection).with_system(hero_selection_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame).with_system(show_hero_selection_system),
        );
    }
}

fn hero_selection_system(
    mut commands: Commands,
    egui: ResMut<EguiContext>,
    mut local_player: Query<(Entity, Option<&mut HeroKind>), (With<Authority>, With<Player>)>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    let (player, current_hero_kind) = local_player.single_mut();

    Area::new("Confirm area")
        .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            ui.add_enabled_ui(current_hero_kind.is_some(), |ui| {
                if ui.button("Confirm").clicked() {
                    ui_state_history.clear();
                    ui_state_history.push(UiState::Hud);
                }
            })
        });

    Window::new("Custom game")
        .anchor(Align2::LEFT_CENTER, (UI_MARGIN, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            if let Some(mut current_hero_kind) = current_hero_kind {
                for kind in HeroKind::iter() {
                    let selected = *current_hero_kind == kind;
                    // TODO: Add hero icon
                    let button = ImageButton::new(TextureId::Managed(0), vec2(32.0, 32.0))
                        .uv(Rect::from_two_pos(WHITE_UV, WHITE_UV))
                        .selected(selected);

                    if ui.add(button).clicked() && selected {
                        *current_hero_kind = kind;
                    };
                }
            } else {
                for hero_kind in HeroKind::iter() {
                    // TODO: Add hero icon
                    let button = ImageButton::new(TextureId::Managed(0), vec2(32.0, 32.0))
                        .uv(Rect::from_two_pos(WHITE_UV, WHITE_UV));

                    if ui.add(button).clicked() {
                        commands.entity(player).insert(hero_kind);
                    };
                }
            }
        });
}

fn show_hero_selection_system(
    mut ui_state_history: ResMut<UiStateHistory>,
    server_settings: Res<ServerSettings>,
) {
    ui_state_history.clear();
    if server_settings.random_heroes {
        // Skip hero selection
        ui_state_history.push(UiState::Hud);
    } else {
        ui_state_history.push(UiState::HeroSelection);
    }
}

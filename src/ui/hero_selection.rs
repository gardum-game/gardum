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

use bevy::prelude::*;
use bevy_egui::{
    egui::{epaint::WHITE_UV, vec2, Align2, Area, ImageButton, Rect, TextureId, Window},
    EguiContext,
};
use iyes_loopless::prelude::*;
use strum::IntoEnumIterator;

use super::{ui_state::UiState, UI_MARGIN};
use crate::core::{
    game_state::GameState, hero::HeroKind, network::server::ServerSettings, player::Player,
    Authority,
};

pub struct HeroSelectionPlugin;

impl Plugin for HeroSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::hero_selection_system.run_in_state(UiState::HeroSelection))
            .add_enter_system(GameState::InGame, Self::show_hero_selection_system);
    }
}

impl HeroSelectionPlugin {
    fn hero_selection_system(
        mut commands: Commands,
        mut egui: ResMut<EguiContext>,
        mut local_player: Query<(Entity, Option<&mut HeroKind>), (With<Authority>, With<Player>)>,
    ) {
        let (player, current_hero_kind) = local_player.single_mut();

        Area::new("Confirm area")
            .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
            .show(egui.ctx_mut(), |ui| {
                ui.add_enabled_ui(current_hero_kind.is_some(), |ui| {
                    if ui.button("Confirm").clicked() {
                        commands.insert_resource(NextState(UiState::Hud));
                    }
                })
            });

        Window::new("Custom game")
            .anchor(Align2::LEFT_CENTER, (UI_MARGIN, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(egui.ctx_mut(), |ui| {
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

    fn show_hero_selection_system(mut commands: Commands, server_settings: Res<ServerSettings>) {
        if server_settings.random_heroes {
            // Skip hero selection
            commands.insert_resource(NextState(UiState::Hud));
        } else {
            commands.insert_resource(NextState(UiState::HeroSelection));
        }
    }
}

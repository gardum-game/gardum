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
use crate::{
    characters::heroes::HeroKind,
    core::{player::Player, AppState, Authority},
};

pub struct HeroSelectionPlugin;

impl Plugin for HeroSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::HeroSelection)
                .with_system(hero_selection_system.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(show_hero_selection_system.system()),
        );
    }
}

fn hero_selection_system(
    mut commands: Commands,
    egui: ResMut<EguiContext>,
    mut player_query: Query<(Entity, Option<&mut HeroKind>), (With<Authority>, With<Player>)>,
    mut ui_state_history: ResMut<UiStateHistory>,
) {
    let (player, hero_kind) = player_query.single_mut();

    Window::new("Custom game")
        .anchor(Align2::LEFT_CENTER, (UI_MARGIN, 0.0))
        .collapsible(false)
        .resizable(false)
        .show(egui.ctx(), |ui| {
            if let Some(mut hero_kind) = hero_kind {
                for kind in HeroKind::iter() {
                    let selected = *hero_kind == kind;
                    // TODO: Add hero icon
                    let button = ImageButton::new(TextureId::Egui, vec2(32.0, 32.0))
                        .uv(Rect::from_two_pos(WHITE_UV, WHITE_UV))
                        .selected(selected);

                    if ui.add(button).clicked() && selected {
                        *hero_kind = kind;
                    };
                }
            } else {
                for kind in HeroKind::iter() {
                    // TODO: Add hero icon
                    let button = ImageButton::new(TextureId::Egui, vec2(32.0, 32.0))
                        .uv(Rect::from_two_pos(WHITE_UV, WHITE_UV));

                    if ui.add(button).clicked() {
                        commands.entity(player).insert(kind);
                    };
                }
            }
        });

    Area::new("Confirm area")
        .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            if ui.button("Confirm").clicked() {
                ui_state_history.clear();
                ui_state_history.push(UiState::Hud);
            }
        });
}

fn show_hero_selection_system(mut ui_state_history: ResMut<UiStateHistory>) {
    ui_state_history.clear();
    ui_state_history.push(UiState::HeroSelection);
}
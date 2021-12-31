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
    egui::{Align2, Area},
    EguiContext,
};

use super::MENU_MARGIN;
use crate::core::AppState;

pub struct BackButtonPlugin;

impl Plugin for BackButtonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_inactive_update(AppState::InGame)
                .with_system(back_button_system.system()),
        )
        .add_system_set(
            SystemSet::on_inactive_update(AppState::MainMenu)
                .with_system(back_button_system.system()),
        );
    }
}

fn back_button_system(
    egui: ResMut<EguiContext>,
    input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    Area::new("Back area")
        .anchor(Align2::LEFT_BOTTOM, (MENU_MARGIN, -MENU_MARGIN))
        .show(egui.ctx(), |ui| {
            if input.just_pressed(KeyCode::Escape) || ui.button("Back").clicked() {
                app_state.pop().unwrap();
            }
        });
}
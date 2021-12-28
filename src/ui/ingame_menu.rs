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

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{Align2, Area, Button},
    EguiContext,
};

use crate::core::AppState;

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGameMenu).with_system(ingame_menu_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(show_ingame_menu_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGameMenu)
                .with_system(hide_ingame_menu_system.system()),
        );
    }
}

fn ingame_menu_system(
    egui: ResMut<EguiContext>,
    mut exit_event: EventWriter<AppExit>,
    mut app_state: ResMut<State<AppState>>,
) {
    Area::new("Main Menu")
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .show(egui.ctx(), |ui| {
            if ui.button("Resume").clicked() {
                app_state.pop().unwrap();
            }
            ui.add_enabled(false, Button::new("Settings"));
            if ui.button("Main menu").clicked() {
                app_state.replace(AppState::MainMenu).unwrap();
            }
            if ui.button("Exit").clicked() {
                exit_event.send(AppExit);
            }
        });
}

fn show_ingame_menu_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.push(AppState::InGameMenu).unwrap();
        keys.reset(KeyCode::Escape);
    }
}

fn hide_ingame_menu_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.pop().unwrap();
        keys.reset(KeyCode::Escape);
    }
}

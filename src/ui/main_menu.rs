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
    egui::{Align2, Area, Button, TextStyle},
    EguiContext,
};

use super::GameMenuState;

const MARGIN: f32 = 20.0;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameMenuState::MainMenu)
            .add_system_set(
                SystemSet::on_update(GameMenuState::MainMenu)
                    .with_system(main_menu_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameMenuState::CustomGameMenu)
                    .with_system(back_button_system.system()),
            );
    }
}

fn main_menu_system(
    egui: ResMut<EguiContext>,
    mut exit_event: EventWriter<AppExit>,
    mut main_menu_state: ResMut<State<GameMenuState>>,
) {
    Area::new("Main Menu")
        .anchor(Align2::LEFT_CENTER, (MARGIN, 0.0))
        .show(egui.ctx(), |ui| {
            ui.add_enabled(false, Button::new("Play").text_style(TextStyle::Heading));
            if ui
                .add(Button::new("Custom game").text_style(TextStyle::Heading))
                .clicked()
            {
                main_menu_state.set(GameMenuState::CustomGameMenu).unwrap();
            }
            ui.add_enabled(
                false,
                Button::new("Characters").text_style(TextStyle::Heading),
            );
            ui.add_enabled(
                false,
                Button::new("Settings").text_style(TextStyle::Heading),
            );
            if ui
                .add(Button::new("Exit").text_style(TextStyle::Heading))
                .clicked()
            {
                exit_event.send(AppExit);
            }
        });
}

fn back_button_system(
    egui: ResMut<EguiContext>,
    input: Res<Input<KeyCode>>,
    mut main_menu_state: ResMut<State<GameMenuState>>,
) {
    Area::new("Back area")
        .anchor(Align2::LEFT_BOTTOM, (MARGIN, -MARGIN))
        .show(egui.ctx(), |ui| {
            if input.just_pressed(KeyCode::Escape) || ui.button("Back").clicked() {
                main_menu_state.set(GameMenuState::MainMenu).unwrap();
            }
        });
}

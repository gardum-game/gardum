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

mod back_button;
mod chat;
mod cursor;
mod custom_game_menu;
mod hero_selection;
mod hud;
mod ingame_menu;
mod main_menu;
mod scoreboard;
mod setting_menu;
pub(super) mod ui_state;

use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike,
};

use back_button::BackButtonPlugin;
use chat::ChatPlugin;
use cursor::CursorPlugin;
use custom_game_menu::CustomGameMenuPlugin;
use hero_selection::HeroSelectionPlugin;
use hud::HudPlugin;
use ingame_menu::InGameMenuPlugin;
use main_menu::MainMenuPlugin;
use scoreboard::ScoreboardPlugin;
use setting_menu::SettingMenuPlugin;
use ui_state::UiStatePlugin;

const UI_MARGIN: f32 = 20.0;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let mut input_map = InputMap::default();
        input_map
            .insert(UiAction::Back, KeyCode::Escape)
            .insert(UiAction::Scoreboard, KeyCode::Tab)
            .insert(UiAction::Chat, KeyCode::Return);

        app.init_resource::<ActionState<UiAction>>()
            .insert_resource(input_map)
            .add_plugin(UiStatePlugin)
            .add_plugin(ChatPlugin)
            .add_plugin(CursorPlugin)
            .add_plugin(SettingMenuPlugin)
            .add_plugin(HeroSelectionPlugin)
            .add_plugin(HudPlugin)
            .add_plugin(ScoreboardPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(CustomGameMenuPlugin)
            .add_plugin(BackButtonPlugin)
            .add_plugin(InGameMenuPlugin);
    }
}

#[derive(Actionlike, Clone, Copy)]
pub(super) enum UiAction {
    Back,
    Scoreboard,
    Chat,
}

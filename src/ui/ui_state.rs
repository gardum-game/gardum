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

use crate::core::game_state::GameState;

pub(super) struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UiState::Empty);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(super) enum UiState {
    Empty,
    MainMenu,
    ServerBrowser,
    SettingsMenu,
    ErrorDialog,
    DirectConnectMenu,
    CrateLobbyMenu,
    LobbyMenu,
    HeroSelection,
    Hud,
    InGameMenu,
}

impl Default for UiState {
    fn default() -> Self {
        Self::Empty
    }
}

impl UiState {
    pub(super) fn previous_state(self, game_state: &State<GameState>) -> Self {
        match self {
            UiState::ServerBrowser => UiState::MainMenu,
            UiState::SettingsMenu => match game_state.current() {
                GameState::Menu => UiState::MainMenu,
                GameState::InGame => UiState::InGameMenu,
                _ => unreachable!(),
            },
            UiState::DirectConnectMenu | UiState::CrateLobbyMenu | UiState::LobbyMenu => {
                UiState::ServerBrowser
            }
            _ => unreachable!("Previous state isn't defined for this state"),
        }
    }
}

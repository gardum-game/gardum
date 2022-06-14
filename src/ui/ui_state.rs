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
use bevy_renet::renet::{RenetClient, RenetServer};
use iyes_loopless::prelude::*;

pub(super) struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        if app.world.get_resource::<RenetClient>().is_some() {
            app.add_loopless_state(UiState::DirectConnectMenu);
        } else if app.world.get_resource::<RenetServer>().is_some() {
            app.add_loopless_state(UiState::LobbyMenu);
        } else {
            app.add_loopless_state(UiState::MainMenu);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(super) enum UiState {
    MainMenu,
    ServerBrowser,
    SettingsMenu,
    DirectConnectMenu,
    LobbyMenu,
    HeroSelection,
    Hud,
    InGameMenu,
}

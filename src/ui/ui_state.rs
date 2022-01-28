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
use derive_more::{Deref, DerefMut};

pub(super) struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(UiState::MainMenu)
            .init_resource::<UiStateHistory>()
            .add_system(update_ui_state);
    }
}

fn update_ui_state(ui_state_history: Res<UiStateHistory>, mut ui_state: ResMut<State<UiState>>) {
    if ui_state_history.is_changed() && !ui_state_history.is_added() {
        let last_state = *ui_state_history
            .last()
            .expect("State history should always contain at least one element");
        ui_state.set(last_state).unwrap();
    }
}

#[derive(Deref, DerefMut)]
pub(super) struct UiStateHistory(pub(super) Vec<UiState>);

impl Default for UiStateHistory {
    fn default() -> Self {
        Self(vec![UiState::MainMenu])
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum UiState {
    MainMenu,
    CustomGameMenu,
    DirectConnectMenu,
    CreateGameMenu,
    LobbyMenu,
    HeroSelection,
    Hud,
    InGameMenu,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_history_updates_state() {
        let mut app = setup_app();
        app.update();

        assert_eq!(
            *app.world
                .get_resource::<State<UiState>>()
                .unwrap()
                .current(),
            UiState::MainMenu,
            "Initial state should be main menu"
        );

        const STATE: UiState = UiState::CustomGameMenu;
        app.world
            .get_resource_mut::<UiStateHistory>()
            .unwrap()
            .push(STATE);

        app.update();

        assert_eq!(
            *app.world
                .get_resource::<State<UiState>>()
                .unwrap()
                .current(),
            STATE,
            "History change should modify current active state"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(UiStatePlugin);
        app
    }
}

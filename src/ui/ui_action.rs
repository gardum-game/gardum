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
use leafwing_input_manager::prelude::*;
use strum::EnumIter;

pub(super) struct UiActionPlugin;

impl Plugin for UiActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui_actions_system);
    }
}

/// Setup player input on game start
fn setup_ui_actions_system(mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map
        .insert(UiAction::Back, KeyCode::Escape)
        .insert(UiAction::Scoreboard, KeyCode::Tab)
        .insert(UiAction::Chat, KeyCode::Return);
    commands.spawn_bundle(InputManagerBundle::<UiAction> {
        input_map,
        ..Default::default()
    });
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
pub(crate) enum UiAction {
    Back,
    Scoreboard,
    Chat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mappings_setup() {
        let mut app = setup_app();

        app.update();

        assert_eq!(
            app.world
                .query::<&InputMap<UiAction>>()
                .iter(&app.world)
                .count(),
            1,
            "UI actions should be created at startup"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(UiActionPlugin);
        app
    }
}

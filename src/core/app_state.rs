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
#[cfg(test)]
use strum::EnumIter;

use super::cli::Opts;

pub(super) struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        let opts = app
            .world
            .get_resource::<Opts>()
            .expect("Command line options should be initialized before app state setting");
        if opts.subcommand.is_some() {
            app.add_state(AppState::InGame);
        } else {
            app.add_state(AppState::Menu);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(EnumIter))]
pub(crate) enum AppState {
    Menu,
    Lobby,
    InGame,
}

#[cfg(test)]
mod tests {
    use crate::core::cli::SubCommand;

    use super::*;

    #[test]
    fn in_game_with_subcommand() {
        let mut app = App::new();
        app.world.insert_resource(Opts {
            subcommand: Some(SubCommand::Connect),
        });
        app.add_plugin(AppStatePlugin);

        assert_eq!(
            *app.world
                .get_resource::<State<AppState>>()
                .unwrap()
                .current(),
            AppState::InGame,
            "State should be in game when launched with a subcommand"
        );
    }

    #[test]
    fn in_menu_without_subcommand() {
        let mut app = App::new();
        app.init_resource::<Opts>();
        app.add_plugin(AppStatePlugin);

        assert_eq!(
            *app.world
                .get_resource::<State<AppState>>()
                .unwrap()
                .current(),
            AppState::Menu,
            "State should be in menu when launched without a subcommand"
        );
    }
}

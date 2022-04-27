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
            app.add_state(GameState::InGame);
        } else {
            app.add_state(GameState::Menu);
        }
        app.add_system_set(
            SystemSet::on_exit(GameState::InGame).with_system(remove_ingame_entities_system),
        );
    }
}

fn remove_ingame_entities_system(
    mut commands: Commands,
    ingame_entities: Query<Entity, With<InGameOnly>>,
) {
    for entity in ingame_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// All entities with this component will be removed after leaving [`InGame`] state
#[derive(Component)]
pub(super) struct InGameOnly;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(EnumIter))]
pub(crate) enum GameState {
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
        let app = setup_app(Opts {
            subcommand: Some(SubCommand::Connect),
        });

        assert_eq!(
            *app.world.resource::<State<GameState>>().current(),
            GameState::InGame,
            "State should be in game when launched with a subcommand"
        );
    }

    #[test]
    fn in_menu_without_subcommand() {
        let app = setup_app(Opts::default());

        assert_eq!(
            *app.world.resource::<State<GameState>>().current(),
            GameState::Menu,
            "State should be in menu when launched without a subcommand"
        );
    }

    #[test]
    fn ingame_entities_cleanup() {
        let mut app = setup_app(Opts {
            subcommand: Some(SubCommand::Connect),
        });

        let child_entity = app.world.spawn().id();
        let ingame_entity = app
            .world
            .spawn()
            .insert(InGameOnly)
            .push_children(&[child_entity])
            .id();
        app.world
            .resource_mut::<State<GameState>>()
            .set(GameState::Menu)
            .expect("State should be swithed to cleanup ingame entities");

        app.update();

        assert!(
            app.world.get_entity(ingame_entity).is_none(),
            "Ingame entity should be despawned after leaving ingame state"
        );
        assert!(
            app.world.get_entity(child_entity).is_none(),
            "Children of ingame entity should be despawned too"
        );
    }

    fn setup_app(opts: Opts) -> App {
        let mut app = App::new();
        app.insert_resource(opts).add_plugin(AppStatePlugin);
        app
    }
}

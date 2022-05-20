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

pub(super) struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Menu).add_system_set(
            SystemSet::on_exit(GameState::InGame).with_system(Self::cleanup_ingame_entities_system),
        );
    }
}

impl AppStatePlugin {
    fn cleanup_ingame_entities_system(
        mut commands: Commands,
        ingame_entities: Query<Entity, With<InGameOnly>>,
    ) {
        for entity in ingame_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// All entities with this component will be removed after leaving [`InGame`] state
#[derive(Component)]
pub(super) struct InGameOnly;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum GameState {
    Menu,
    InGame,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingame_entities_cleanup() {
        let mut app = setup_app();
        app.world
            .resource_mut::<State<GameState>>()
            .set(GameState::InGame)
            .expect("In-game entities should be spawned in a different state");
        let child_entity = app.world.spawn().id();
        let ingame_entity = app
            .world
            .spawn()
            .insert(InGameOnly)
            .push_children(&[child_entity])
            .id();

        app.update();

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
            "Children of ingame entity should be despawned with its parent"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(AppStatePlugin);
        app
    }
}

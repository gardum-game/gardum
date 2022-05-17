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
use derive_more::Display;
use leafwing_input_manager::{prelude::InputMap, Actionlike};
use serde::{Deserialize, Serialize};

use super::{
    game_state::GameState,
    player::Player,
    settings::{SettingApplyEvent, Settings},
    Authority,
};

pub(super) struct ControlActionsPlugin;

impl Plugin for ControlActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::load_mappings_system).add_system_set(
            SystemSet::on_enter(GameState::InGame).with_system(Self::setup_mappings_system),
        );
    }
}

impl ControlActionsPlugin {
    fn load_mappings_system(
        mut apply_events: EventReader<SettingApplyEvent>,
        mut local_player: Query<&mut InputMap<ControlAction>, With<Authority>>,
        settings: Res<Settings>,
    ) {
        if apply_events.iter().next().is_some() {
            if let Ok(mut mappings) = local_player.get_single_mut() {
                *mappings = settings.controls.mappings.clone();
            }
        }
    }

    /// Setup player input on game start
    fn setup_mappings_system(
        mut commands: Commands,
        settings: Res<Settings>,
        local_player: Query<Entity, (With<Authority>, With<Player>)>,
    ) {
        let local_player = local_player.single();
        commands
            .entity(local_player)
            .insert(settings.controls.mappings.clone());
    }
}

#[derive(Actionlike, Component, Clone, Copy, PartialEq, Hash, Display, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub(crate) enum ControlAction {
    // Movement
    Forward,
    Backward,
    Left,
    Right,
    Jump,

    // Abilities activation
    BaseAttack,
    Ability1,
    Ability2,
    Ability3,
    Ultimate,
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use super::*;

    #[test]
    fn mappings_applies() {
        let mut app = setup_app();
        let player = app.world.spawn().insert(Authority).insert(Player).id();

        app.update();

        let mappings = app
            .world
            .entity(player)
            .get::<InputMap<ControlAction>>()
            .expect("Mappings should be added to the local player");

        let settings = app.world.resource::<Settings>();
        assert_eq!(
            settings.controls.mappings, *mappings,
            "Added mappings should the same as in settings"
        );

        // Change settings to test reloading
        let mut settings = app.world.resource_mut::<Settings>();
        settings
            .controls
            .mappings
            .insert(ControlAction::Jump, KeyCode::Q);

        let mut apply_events = app.world.resource_mut::<Events<SettingApplyEvent>>();
        apply_events.send(SettingApplyEvent);

        app.update();

        let settings = app.world.resource::<Settings>();
        let mappings = app
            .world
            .entity(player)
            .get::<InputMap<ControlAction>>()
            .unwrap();
        assert_eq!(
            settings.controls.mappings, *mappings,
            "Mappings should be updated on apply event"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_state(GameState::InGame)
            .add_event::<SettingApplyEvent>()
            .init_resource::<Settings>()
            .add_plugin(ControlActionsPlugin);
        app
    }
}

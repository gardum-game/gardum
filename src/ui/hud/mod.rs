/*
 *  Copyright © 2021-2022 Hennadii Chernyshchyk <genaloner@gmail.com>
 *
 *  This file is part of Gardum.
 *
 *  Gardum is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  Gardum is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with Gardum. If not, see <https://www.gnu.org/licenses/>.
 */

mod ability_icon;
mod health_bar;

use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{
    egui::{Align2, Area, TextureId},
    EguiContext,
};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{chat_window::ChatWindowPlugin, ui_actions::UiAction, ui_state::UiState, UI_MARGIN};
use crate::core::{
    ability::{Abilities, IconPath},
    control_actions::ControlAction,
    cooldown::Cooldown,
    health::Health,
    Authority,
};
use ability_icon::AbilityIcon;
use health_bar::HealthBar;

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.world
            .resource_mut::<ToggleActions<ControlAction>>()
            .enabled = false; // Should be initialized in disabled state and enabled only on hud

        app.add_system(Self::health_and_abilities_system.run_in_state(UiState::Hud))
            .add_system(
                Self::show_ingame_menu_system
                    .run_in_state(UiState::Hud)
                    .after(ChatWindowPlugin::chat_system),
            )
            .add_enter_system(UiState::Hud, Self::enable_control_actions_system)
            .add_enter_system(UiState::Hud, Self::hide_cursor_system)
            .add_exit_system(UiState::Hud, Self::disable_control_actions_system)
            .add_exit_system(UiState::Hud, Self::show_cursor_system);
    }
}

impl HudPlugin {
    fn health_and_abilities_system(
        mut ability_icons: Local<HashMap<Handle<Image>, TextureId>>,
        asset_server: Res<AssetServer>,
        mut egui: ResMut<EguiContext>,
        local_character: Query<(&Abilities, &Health), With<Authority>>,
        cooldowns: Query<&Cooldown>,
        icon_paths: Query<&IconPath>,
    ) {
        let (abilities, health) = match local_character.get_single() {
            Ok(result) => result,
            Err(_) => return,
        };

        for ability in abilities.iter() {
            let icon_path = icon_paths.get(*ability).unwrap();
            let image = asset_server.load(icon_path.0);
            let texture_id = egui.add_image(image.as_weak());
            ability_icons.insert(image, texture_id);
        }

        Area::new("Health and abilities")
            .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
            .show(egui.ctx_mut(), |ui| {
                ui.set_width(300.0);
                ui.add(HealthBar::new(health.current, health.max));
                ui.horizontal(|ui| {
                    for (ability, texture_id) in abilities.iter().zip(ability_icons.values()) {
                        let cooldown = cooldowns.get(*ability).ok();
                        ui.add(AbilityIcon::new(*texture_id, cooldown));
                    }
                })
            });
    }

    fn enable_control_actions_system(mut toggle_actions: ResMut<ToggleActions<ControlAction>>) {
        toggle_actions.enabled = true;
    }

    fn disable_control_actions_system(mut toggle_actions: ResMut<ToggleActions<ControlAction>>) {
        toggle_actions.enabled = false;
    }

    fn hide_cursor_system(mut windows: ResMut<Windows>) {
        let window = windows.primary_mut();

        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    fn show_cursor_system(mut windows: ResMut<Windows>) {
        let window = windows.primary_mut();

        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }

    fn show_ingame_menu_system(
        mut commands: Commands,
        mut action_state: ResMut<ActionState<UiAction>>,
    ) {
        if action_state.just_pressed(UiAction::Back) {
            action_state.consume(UiAction::Back);
            commands.insert_resource(NextState(UiState::InGameMenu));
        }
    }
}

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

mod ability_icon;
mod health_bar;

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};
use leafwing_input_manager::plugin::ToggleActions;

use super::{ui_state::UiState, UI_MARGIN};
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
        app.add_system_set(
            SystemSet::on_update(UiState::Hud).with_system(health_and_abilities_system),
        )
        .add_system_set(
            SystemSet::on_enter(UiState::Hud)
                .with_system(enable_control_actions_system)
                .with_system(hide_cursor_system),
        )
        .add_system_set(
            SystemSet::on_exit(UiState::Hud)
                .with_system(disable_control_actions_system)
                .with_system(show_cursor_system),
        );
    }
}

fn health_and_abilities_system(
    local_character: Query<(&Abilities, &Health), With<Authority>>,
    cooldowns: Query<&Cooldown>,
    icon_paths: Query<&IconPath>,
    mut ability_images: Local<Vec<Handle<Image>>>,
    asset_server: Res<AssetServer>,
    mut egui: ResMut<EguiContext>,
) {
    let (abilities, health) = match local_character.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    for (i, ability) in abilities.iter().enumerate() {
        let icon_path = icon_paths.get(*ability).unwrap();
        let image = asset_server.load(icon_path.0);
        if let Some(current_image) = ability_images.get_mut(i) {
            if image != *current_image {
                egui.add_image(current_image.as_weak());
                *current_image = image;
            }
        } else {
            egui.add_image(image.as_weak());
            ability_images.push(image);
        }
    }

    Area::new("Health and abilities")
        .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            ui.set_width(300.0);
            ui.add(HealthBar::new(health.current, health.max));
            ui.horizontal(|ui| {
                for (ability, image) in abilities.iter().zip(ability_images.iter().by_ref()) {
                    let cooldown = cooldowns.get(*ability).ok();
                    let image_id = egui.image_id(image).unwrap();
                    ui.add(AbilityIcon::new(image_id, cooldown));
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
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}

fn show_cursor_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(false);
    window.set_cursor_visibility(true);
}

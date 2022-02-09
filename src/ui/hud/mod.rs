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
    egui::{Align2, Area, Image, TextureId},
    EguiContext,
};

use super::{ui_state::UiState, UI_MARGIN};
use crate::core::{
    ability::{Abilities, Cooldown, IconPath},
    health::Health,
    Local,
};
use ability_icon::AbilityIcon;
use health_bar::HealthBar;

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(UiState::Hud)
                .with_system(ability_icons_texture_system)
                .with_system(health_and_abilities_system),
        );
    }
}

fn health_and_abilities_system(
    local_character: Query<(&Abilities, &Health), With<Local>>,
    ability_cooldowns: Query<&Cooldown>,
    egui: ResMut<EguiContext>,
) {
    let (abilities, health) = match local_character.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    Area::new("Health and abilities")
        .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            ui.set_width(300.0);
            ui.add(HealthBar::new(health.current, health.max));
            ui.horizontal(|ui| {
                for (slot, ability) in abilities.iter().enumerate() {
                    let image = Image::new(TextureId::User(slot as u64), [64.0, 64.0]);
                    let cooldown = ability_cooldowns.get(*ability).ok();
                    ui.add(AbilityIcon::new(image, cooldown));
                }
            })
        });
}

fn ability_icons_texture_system(
    new_local_abilities: Query<&Abilities, Added<Local>>,
    icons: Query<&IconPath>,
    assets: Res<AssetServer>,
    mut egui: ResMut<EguiContext>,
) {
    let abilities = match new_local_abilities.get_single() {
        Ok(abilities) => abilities,
        Err(_) => return,
    };

    for (i, ability) in abilities.iter().enumerate() {
        if let Ok(icon) = icons.get(*ability) {
            egui.set_egui_texture(i as u64, assets.load(icon.0));
        } else {
            egui.remove_egui_texture(i as u64);
        }
    }
}

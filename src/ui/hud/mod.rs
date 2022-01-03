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

mod health_bar;

use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};

use super::UI_MARGIN;
use crate::{
    characters::health::Health,
    core::{AppState, Authority},
};
use health_bar::HealthBar;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(health_and_abilities.system()),
        );
    }
}

fn health_and_abilities(health_query: Query<&Health, With<Authority>>, egui: ResMut<EguiContext>) {
    Area::new("Health and abilities")
        .anchor(Align2::CENTER_BOTTOM, (0.0, -UI_MARGIN))
        .show(egui.ctx(), |ui| {
            ui.set_width(300.0);
            let health = health_query.single().unwrap();
            ui.add(HealthBar::new(health.current, health.max));
        });
}

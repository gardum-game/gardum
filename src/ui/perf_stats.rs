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

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};

use super::UI_MARGIN;
use crate::core::settings::{SettingApplyEvent, Settings};

pub(super) struct PerfStatsPlugin;

impl Plugin for PerfStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::perf_stats_system);
    }
}

impl PerfStatsPlugin {
    fn perf_stats_system(
        mut enabled: Local<bool>,
        mut apply_events: EventReader<SettingApplyEvent>,
        diagnostics: Res<Diagnostics>,
        settings: Res<Settings>,
        egui: Res<EguiContext>,
    ) {
        if apply_events.iter().next().is_some() || settings.is_added() {
            *enabled = settings.video.perf_stats;
        }
        if !*enabled {
            return;
        }

        let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();
        Area::new("Performance stats")
            .anchor(Align2::LEFT_TOP, (UI_MARGIN, UI_MARGIN))
            .show(egui.ctx(), |ui| {
                if let Some(fps) = fps.value() {
                    ui.strong(format!("FPS: {:.0}", fps));
                }
            });
    }
}

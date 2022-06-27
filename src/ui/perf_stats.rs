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

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{
    egui::{Align2, Area},
    EguiContext,
};
use bevy_renet::renet::RenetClient;

use super::UI_MARGIN;
use crate::core::settings::{Settings, SettingsApplied};

pub(super) struct PerfStatsPlugin;

impl Plugin for PerfStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::perf_stats_system);
    }
}

impl PerfStatsPlugin {
    fn perf_stats_system(
        mut enabled: Local<bool>,
        mut apply_events: EventReader<SettingsApplied>,
        diagnostics: Res<Diagnostics>,
        settings: Res<Settings>,
        client: Option<Res<RenetClient>>,
        mut egui: ResMut<EguiContext>,
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
            .show(egui.ctx_mut(), |ui| {
                if let Some(fps) = fps.value() {
                    ui.strong(format!("FPS: {:.0}", fps));
                }
                if let Some(client) = client {
                    let network_info = client.network_info();
                    ui.strong(format!(
                        "RTT: {:.02} ⬇ {:.02} kbps ⬆ {:.02} kbps",
                        network_info.rtt, network_info.received_kbps, network_info.sent_kbps,
                    ));
                }
            });
    }
}

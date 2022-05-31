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
use leafwing_input_manager::prelude::InputMap;
use serde::{Deserialize, Serialize};
use standard_paths::{LocationType, StandardPaths};
use std::{fs, path::PathBuf};

use super::control_actions::ControlAction;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingsApplied>()
            .insert_resource(Settings::read())
            .add_system(Self::write_system);
    }
}

impl SettingsPlugin {
    fn write_system(mut apply_events: EventReader<SettingsApplied>, settings: Res<Settings>) {
        if apply_events.iter().next().is_some() {
            settings.write();
        }
    }
}

/// An event that applies the specified settings in the [`Settings`] resource.
pub(crate) struct SettingsApplied;

#[derive(Default, Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(default)]
pub(crate) struct Settings {
    pub(crate) video: VideoSettings,
    pub(crate) controls: ControlsSettings,
}

impl Settings {
    /// Creates [`Settings`] from the application settings file.
    /// Will be initialed with defaults if the file does not exist.
    fn read() -> Settings {
        match fs::read_to_string(Settings::file_path()) {
            Ok(content) => {
                serde_json::from_str::<Settings>(&content).expect("Unable to parse setting file")
            }
            Err(_) => Settings::default(),
        }
    }

    /// Serialize [`Settings`] on disk under [`self.file_path`].
    fn write(&self) {
        let content = serde_json::to_string_pretty(&self).expect("Unable to serialize settings");
        fs::write(Settings::file_path(), content).expect("Unable to write settings");
    }

    fn file_path() -> PathBuf {
        let standard_paths = StandardPaths::default();
        // Use temp directory in tests
        let mut location = standard_paths
            .writable_location(if cfg!(test) {
                LocationType::TempLocation
            } else {
                LocationType::AppConfigLocation
            })
            .expect("Unable to get application settings directory");

        fs::create_dir_all(&location).expect("Unable to create applicaiton settings directory");

        location.push(env!("CARGO_PKG_NAME"));
        location.set_extension("json");
        location
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(default)]
pub(crate) struct VideoSettings {
    pub(crate) msaa: u32,
    pub(crate) perf_stats: bool,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            msaa: 1,
            perf_stats: false,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(default)]
pub(crate) struct ControlsSettings {
    pub(crate) mappings: InputMap<ControlAction>,
}

impl Default for ControlsSettings {
    fn default() -> Self {
        let mut input = InputMap::default();
        input
            .insert(ControlAction::Forward, KeyCode::W)
            .insert(ControlAction::Backward, KeyCode::S)
            .insert(ControlAction::Left, KeyCode::A)
            .insert(ControlAction::Right, KeyCode::D)
            .insert(ControlAction::Jump, KeyCode::Space)
            .insert(ControlAction::BaseAttack, MouseButton::Left)
            .insert(ControlAction::Ability1, KeyCode::Q)
            .insert(ControlAction::Ability2, KeyCode::E)
            .insert(ControlAction::Ability3, KeyCode::LShift)
            .insert(ControlAction::Ultimate, KeyCode::R);

        Self { mappings: input }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use super::*;

    #[test]
    fn read_write() {
        let mut app = App::new();
        app.add_plugin(SettingsPlugin);

        let mut settings = app.world.resource_mut::<Settings>();
        let file_path = Settings::file_path();
        assert!(
            !file_path.exists(),
            "Settings file shouldn't be created on startup"
        );
        assert_eq!(
            *settings,
            Settings::default(),
            "Settings should be defaulted if settings file does not exist"
        );

        // Modify settings
        settings.video.msaa += 1;

        let mut apply_events = app.world.resource_mut::<Events<SettingsApplied>>();
        apply_events.send(SettingsApplied);

        app.update();

        let settings = app.world.resource::<Settings>();
        assert!(
            file_path.exists(),
            "Configuration file should be created on apply event"
        );

        let loaded_settings = Settings::read();
        assert_eq!(
            settings.video, loaded_settings.video,
            "Loaded settings should be equal to saved"
        );

        fs::remove_file(file_path).expect("Saved file should be removed after the test");
    }
}

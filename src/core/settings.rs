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
use bevy_hikari::GiConfig;
#[cfg(test)]
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use standard_paths::{LocationType, StandardPaths};
use std::{fs, path::PathBuf};

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingApplyEvent>()
            .insert_resource(Settings::new())
            .add_system(apply_video_settings_system)
            .add_system(write_settings_system);
    }
}

pub(crate) struct SettingApplyEvent;

fn apply_video_settings_system(
    mut commands: Commands,
    mut apply_events: EventReader<SettingApplyEvent>,
    settings: Res<Settings>,
) {
    if apply_events.iter().next().is_some() || settings.is_added() {
        commands.insert_resource(Msaa {
            samples: settings.video.msaa,
        });
        commands.insert_resource(GiConfig {
            enabled: settings.video.global_illumination,
        });
    }
}

fn write_settings_system(
    mut apply_events: EventReader<SettingApplyEvent>,
    settings: Res<Settings>,
) {
    if apply_events.iter().next().is_some() {
        settings.write();
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
#[serde(default)]
pub(crate) struct Settings {
    pub(crate) video: VideoSettings,

    #[serde(skip)]
    file_path: PathBuf,
}

impl Settings {
    /// Creates [`Settings`] from the application settings file.
    /// Will be initialed with defaults if the file does not exist.
    fn new() -> Settings {
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

        // Generate ranom files for each tests to avoid access to the same file from multiply tests
        #[cfg(test)]
        location.push(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect::<String>(),
        );
        #[cfg(not(test))]
        location.push(env!("CARGO_PKG_NAME"));
        location.set_extension("toml");

        Settings::from_file(location)
    }

    /// Creates [`Settings`] from the specified file.
    /// Will be initialed with defaults if the file does not exist.
    fn from_file(file_path: PathBuf) -> Settings {
        match fs::read_to_string(&file_path) {
            Ok(content) => Settings {
                file_path,
                ..toml::from_str::<Settings>(&content).expect("Unable to parse setting file")
            },
            Err(_) => Settings {
                file_path,
                ..Default::default()
            },
        }
    }

    /// Serialize [`Settings`] on disk under [`self.file_path`].
    fn write(&self) {
        let content = toml::to_string_pretty(&self).expect("Unable to serialize settings");
        fs::write(&self.file_path, content).expect("Unable to write settings");
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(default)]
pub(crate) struct VideoSettings {
    pub(crate) msaa: u32,
    pub(crate) global_illumination: bool,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            msaa: 1,
            global_illumination: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::Events;

    use super::*;

    #[test]
    fn read_write() {
        let mut app = setup_app();

        let mut settings = app.world.get_resource_mut::<Settings>().unwrap();
        assert!(
            !settings.file_path.exists(),
            "Settings file shouldn't be created on startup"
        );
        assert_eq!(
            settings.video,
            VideoSettings::default(),
            "Video settings should be defaulted if settings file does not exist"
        );

        // Modify settings
        settings.video.msaa += 1;

        let mut apply_events = app
            .world
            .get_resource_mut::<Events<SettingApplyEvent>>()
            .unwrap();
        apply_events.send(SettingApplyEvent);

        app.update();

        let settings = app.world.get_resource::<Settings>().unwrap();
        assert!(
            settings.file_path.exists(),
            "Configuration file should be created on apply event"
        );

        let loaded_settings = Settings::from_file(settings.file_path.clone());
        assert_eq!(
            settings.video, loaded_settings.video,
            "Loaded settings should be equal to saved"
        );

        fs::remove_file(&settings.file_path).expect("Saved file should be removed after the test");
    }

    #[test]
    fn video_settings_applies() {
        let mut app = setup_app();
        app.update();

        let msaa = app.world.get_resource::<Msaa>().unwrap().clone();
        let mut settings = app.world.get_resource_mut::<Settings>().unwrap();
        assert_eq!(
            settings.video.msaa, msaa.samples,
            "MSAA setting should be loaded at startup"
        );

        settings.video.msaa += 1;

        let mut apply_events = app
            .world
            .get_resource_mut::<Events<SettingApplyEvent>>()
            .unwrap();
        apply_events.send(SettingApplyEvent);

        app.update();

        let settings = app.world.get_resource::<Settings>().unwrap();
        let msaa = app.world.get_resource::<Msaa>().unwrap();
        assert_eq!(
            settings.video.msaa, msaa.samples,
            "MSAA setting should be updated on apply event"
        );

        fs::remove_file(&settings.file_path).expect("Saved file should be removed after the test");
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(SettingsPlugin);
        app
    }
}

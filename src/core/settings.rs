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
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingApplyEvent>()
            .insert_resource(Settings::read())
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
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(default)]
pub(crate) struct Settings {
    pub(crate) video: VideoSettings,
}

impl Settings {
    fn read() -> Settings {
        let config_path = Settings::config_path();
        match fs::read_to_string(config_path) {
            Ok(content) => {
                toml::from_str::<Settings>(&content).expect("Unable to parse setting file")
            }
            Err(_) => Settings::default(),
        }
    }

    fn write(&self) {
        let config_path = Settings::config_path();
        let content = toml::to_string_pretty(&self).expect("Unable to serialize settings");
        fs::write(config_path, content).expect("Unable to write settings");
    }

    fn config_path() -> PathBuf {
        let current_exe = env::current_exe().expect("Unable to get current executable location");
        let current_folder = current_exe
            .parent()
            .expect("Unable to get executable folder");

        // Do not overwrite normal configuration file during tests
        if cfg!(test) {
            current_folder.join("config.toml")
        } else {
            current_folder.join("test-config.toml")
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(default)]
pub(crate) struct VideoSettings {
    pub(crate) msaa: u32,
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self { msaa: 1 }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::Events;

    use super::*;

    #[test]
    fn read_write() {
        let mut app = setup_app();
        let config_path = Settings::config_path();
        assert!(
            !config_path.exists(),
            "Configuration file shouldn't be created on startup"
        );
        assert_eq!(
            *app.world.get_resource::<Settings>().unwrap(),
            Settings::default(),
            "When the configuration file does not exist, all values defaulted"
        );

        // Modify settings
        let mut settings = app.world.get_resource_mut::<Settings>().unwrap();
        settings.video.msaa += 1;

        let mut hit_events = app
            .world
            .get_resource_mut::<Events<SettingApplyEvent>>()
            .unwrap();
        hit_events.send(SettingApplyEvent);

        app.update();

        assert!(
            config_path.exists(),
            "Configuration file should be created on apply event"
        );

        let loaded_settings = Settings::read();
        assert_eq!(
            *app.world.get_resource::<Settings>().unwrap(),
            loaded_settings,
            "Loaded settings should be equal to saved"
        );
        fs::remove_file(config_path).expect("Saved file should be removed after the test");
    }

    #[test]
    fn video_settings_applies() {
        let mut app = setup_app();
        app.update();

        let msaa = app.world.get_resource::<Msaa>().unwrap();
        let settings = app.world.get_resource::<Settings>().unwrap();
        assert_eq!(
            settings.video.msaa, msaa.samples,
            "MSAA setting should be loaded at startup"
        );

        let mut settings = app.world.get_resource_mut::<Settings>().unwrap();
        settings.video.msaa += 1;

        let mut hit_events = app
            .world
            .get_resource_mut::<Events<SettingApplyEvent>>()
            .unwrap();
        hit_events.send(SettingApplyEvent);

        app.update();

        let settings = app.world.get_resource::<Settings>().unwrap();
        let msaa = app.world.get_resource::<Msaa>().unwrap();
        assert_eq!(
            settings.video.msaa, msaa.samples,
            "MSAA setting should be updated on apply event"
        );
        fs::remove_file(Settings::config_path())
            .expect("Saved file should be removed after the test");
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(SettingsPlugin);
        app
    }
}

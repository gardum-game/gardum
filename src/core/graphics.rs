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

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::settings::{Settings, SettingsApplied};

pub(super) struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::apply_graphics_system)
            .add_system(Self::apply_graphics_system.run_on_event::<SettingsApplied>());
    }
}

impl GraphicsPlugin {
    fn apply_graphics_system(mut commands: Commands, settings: Res<Settings>) {
        commands.insert_resource(Msaa {
            samples: settings.video.msaa,
        });
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use super::*;

    #[test]
    fn graphics_applies() {
        let mut app = App::new();
        app.add_plugin(TestPlayerPlugin);

        app.update();

        let samples = app.world.resource::<Msaa>().samples;
        let mut settings = app.world.resource_mut::<Settings>();
        assert_eq!(
            settings.video.msaa, samples,
            "MSAA setting should be loaded at startup"
        );

        settings.video.msaa += 1;

        let mut apply_events = app.world.resource_mut::<Events<SettingsApplied>>();
        apply_events.send(SettingsApplied);

        app.update();

        let samples = app.world.resource::<Msaa>().samples;
        let settings = app.world.resource::<Settings>();
        assert_eq!(
            settings.video.msaa, samples,
            "MSAA setting should be updated on apply event"
        );
    }

    struct TestPlayerPlugin;

    impl Plugin for TestPlayerPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<Settings>()
                .add_event::<SettingsApplied>()
                .add_plugin(GraphicsPlugin);
        }
    }
}

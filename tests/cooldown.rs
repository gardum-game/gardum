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
use std::time::Duration;

use gardum::{
    characters::cooldown::{Cooldown, CooldownPlugin},
    core::AppState,
};

#[test]
fn cooldown_ticks() {
    let mut app = setup_app();

    let mut cooldown = Cooldown::from_secs(1);
    cooldown.reset(); // Activate cooldown
    let cooldown_entity = app.world.spawn().insert(cooldown).id();

    app.update();
    app.update();
    let cooldown = app.world.get::<Cooldown>(cooldown_entity).unwrap();
    assert!(
        cooldown.elapsed() > Duration::default(),
        "Cooldown should tick"
    );
}

fn setup_app() -> App {
    let mut app = App::new();
    app.add_state(AppState::InGame)
        .add_plugins(MinimalPlugins)
        .add_plugin(CooldownPlugin);
    app
}

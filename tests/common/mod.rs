/*
 *  Copyright © 2021 Hennadii Chernyshchyk <genaloner@gmail.com>
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
    app::Events,
    ecs::component::Component,
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
        ElementState,
    },
    prelude::*,
};

pub fn simulate_key_press(app: &mut App, code: KeyCode) {
    let mut events = app
        .world
        .get_resource_mut::<Events<KeyboardInput>>()
        .unwrap();

    events.send(KeyboardInput {
        scan_code: 0,
        key_code: Some(code),
        state: ElementState::Pressed,
    });

    app.update();
}

pub fn simulate_mouse_press(app: &mut App, button: MouseButton) {
    let mut events = app
        .world
        .get_resource_mut::<Events<MouseButtonInput>>()
        .unwrap();

    events.send(MouseButtonInput {
        button,
        state: ElementState::Pressed,
    });

    app.update();
}

pub fn simulate_mouse_movement(app: &mut App, delta: Vec2) {
    let mut events = app.world.get_resource_mut::<Events<MouseMotion>>().unwrap();

    events.send(MouseMotion { delta });

    app.update();
}

pub fn events_count<T: Component>(world: &mut World) -> usize {
    let events = world.get_resource::<Events<T>>().unwrap();
    let mut reader = events.get_reader();
    reader.iter(&events).count()
}

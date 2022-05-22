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
    ecs::system::SystemParam,
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState},
    prelude::*,
};
use leafwing_input_manager::user_input::InputButton;

/// Helper for collecting input
#[derive(SystemParam)]
pub(super) struct InputEvents<'w, 's> {
    keys: EventReader<'w, 's, KeyboardInput>,
    mouse_buttons: EventReader<'w, 's, MouseButtonInput>,
    gamepad_events: EventReader<'w, 's, GamepadEvent>,
}

impl InputEvents<'_, '_> {
    pub(super) fn input_button(&mut self) -> Option<InputButton> {
        if let Some(keyboard_input) = self.keys.iter().next() {
            if keyboard_input.state == ElementState::Released {
                if let Some(key_code) = keyboard_input.key_code {
                    return Some(key_code.into());
                }
            }
        }

        if let Some(mouse_input) = self.mouse_buttons.iter().next() {
            if mouse_input.state == ElementState::Released {
                return Some(mouse_input.button.into());
            }
        }

        if let Some(GamepadEvent(_, event_type)) = self.gamepad_events.iter().next() {
            if let GamepadEventType::ButtonChanged(button, strength) = event_type.to_owned() {
                if strength <= 0.5 {
                    return Some(button.into());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::event::Events, ecs::system::SystemState, input::InputPlugin};

    use super::*;

    #[test]
    fn input_events_reads_keyboard() {
        let mut app = setup_app();
        const KEY: KeyCode = KeyCode::Space;
        let mut keyboard_input = app.world.resource_mut::<Events<KeyboardInput>>();
        keyboard_input.send(KeyboardInput {
            scan_code: 0,
            key_code: Some(KEY),
            state: ElementState::Released,
        });

        let mut system_state: SystemState<InputEvents> = SystemState::new(&mut app.world);
        let mut input_events = system_state.get_mut(&mut app.world);
        let input_button = input_events
            .input_button()
            .expect("Input button should be detected");
        assert_eq!(
            input_button,
            InputButton::Keyboard(KEY),
            "Input button should be equal to the released keyboard key"
        );
    }

    #[test]
    fn input_events_reads_mouse() {
        let mut app = setup_app();
        const BUTTON: MouseButton = MouseButton::Right;
        let mut mouse_button = app.world.resource_mut::<Events<MouseButtonInput>>();
        mouse_button.send(MouseButtonInput {
            button: BUTTON,
            state: ElementState::Released,
        });

        let mut system_state: SystemState<InputEvents> = SystemState::new(&mut app.world);
        let mut input_events = system_state.get_mut(&mut app.world);
        let input_button = input_events
            .input_button()
            .expect("Input button should be detected");
        assert_eq!(
            input_button,
            InputButton::Mouse(BUTTON),
            "Input button should be equal to the released mouse button"
        );
    }

    #[test]
    fn input_events_reads_gamepad() {
        let mut app = setup_app();
        const BUTTON: GamepadButtonType = GamepadButtonType::Z;
        const PRESSED_STRENGTH: f32 = 0.6;
        let mut gamepad_events = app.world.resource_mut::<Events<GamepadEvent>>();
        gamepad_events.send(GamepadEvent(
            Gamepad(0),
            GamepadEventType::ButtonChanged(BUTTON, PRESSED_STRENGTH),
        ));

        let mut system_state: SystemState<InputEvents> = SystemState::new(&mut app.world);
        let mut input_events = system_state.get_mut(&mut app.world);
        assert_eq!(
            input_events.input_button(),
            None,
            "Input button shouldn't be detected when pressed strength is {PRESSED_STRENGTH}"
        );

        const RELEASED_STRENGTH: f32 = 0.5;
        let mut gamepad_events = app.world.resource_mut::<Events<GamepadEvent>>();
        gamepad_events.send(GamepadEvent(
            Gamepad(0),
            GamepadEventType::ButtonChanged(BUTTON, RELEASED_STRENGTH),
        ));

        let mut input_events = system_state.get_mut(&mut app.world);
        let input_button = input_events
            .input_button()
            .expect("Input button should be detected with {RELEASED_STRENGTH} strength");
        assert_eq!(
            input_button,
            InputButton::Gamepad(BUTTON),
            "Input button should be equal to the released gamepad button"
        );
    }

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugin(InputPlugin);
        app
    }
}

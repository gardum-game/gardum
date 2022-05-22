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

use super::ui_state::UiState;

pub(super) struct ErrorMessagePlugin;

impl Plugin for ErrorMessagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ErrorMessage>()
            .add_system(Self::cleanup_system);
    }
}

impl ErrorMessagePlugin {
    fn cleanup_system(
        mut previous_state: Local<UiState>,
        mut error_message: ResMut<ErrorMessage>,
        ui_state: Res<State<UiState>>,
    ) {
        if *previous_state != *ui_state.current() {
            *previous_state = *ui_state.current();
            error_message.clear();
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub(super) struct ErrorMessage(pub(super) String);

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

mod direct_connect_menu;
mod lobby_menu;
mod server_browser;

use bevy::prelude::*;

use direct_connect_menu::DirectConnectMenuPlugin;
use lobby_menu::LobbyMenuPlugin;
use server_browser::ServerBrowserPlugin;

pub(super) struct CustomGameMenuPlugin;

impl Plugin for CustomGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ServerBrowserPlugin)
            .add_plugin(DirectConnectMenuPlugin)
            .add_plugin(LobbyMenuPlugin);
    }
}

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

pub(crate) mod client;
pub(crate) mod message;
pub(crate) mod server;

use bevy::prelude::*;

use bevy_renet::renet::{ChannelConfig, ReliableChannelConfig, NETCODE_KEY_BYTES};
use client::ClientPlugin;
use message::MessagePlugin;
use server::ServerPlugin;

pub(crate) const DEFAULT_PORT: u16 = 4761;
pub(crate) const MAX_PORT: u16 = 65535;
const PUBLIC_GAME_KEY: [u8; NETCODE_KEY_BYTES] = [0; NETCODE_KEY_BYTES];
const PROTOCOL_ID: u64 = 7;

pub(super) struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(NetworkingState::NoSocket)
            .add_plugin(ServerPlugin)
            .add_plugin(ClientPlugin)
            .add_plugin(MessagePlugin);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum NetworkingState {
    NoSocket,
    Connecting,
    Connected,
    Hosting,
}

enum Channels {
    Reliable,
}

impl Channels {
    fn id(&self) -> u8 {
        match self {
            Channels::Reliable => 0,
        }
    }

    fn config() -> Vec<ChannelConfig> {
        let reliable_channel = ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: Channels::Reliable.id(),
            ..Default::default()
        });

        vec![reliable_channel]
    }
}

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

mod chat;
pub(crate) mod client;
pub(crate) mod message;
pub(crate) mod server;
mod unreliable_message;

use bevy::prelude::*;
use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, UnreliableChannelConfig, NETCODE_KEY_BYTES,
};
use iyes_loopless::prelude::*;

use chat::ChatPlugin;
use client::ClientPlugin;
use message::MessagePlugin;
use server::ServerPlugin;
use unreliable_message::UnreliableMessagePlugin;

pub(crate) const DEFAULT_PORT: u16 = 4761;
pub(crate) const MAX_PORT: u16 = 65535;
pub(crate) const SERVER_ID: u64 = 0;
const PUBLIC_GAME_KEY: [u8; NETCODE_KEY_BYTES] = [0; NETCODE_KEY_BYTES];
const PROTOCOL_ID: u64 = 7;

pub(super) struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(NetworkingState::NoSocket)
            .add_plugin(ServerPlugin)
            .add_plugin(ClientPlugin)
            .add_plugin(MessagePlugin)
            .add_plugin(UnreliableMessagePlugin)
            .add_plugin(ChatPlugin);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum NetworkingState {
    NoSocket,
    Connecting,
    Connected,
    Hosting,
}

pub(crate) enum Channel {
    Reliable,
    Unreliable,
}

impl Channel {
    pub(crate) fn id(&self) -> u8 {
        match self {
            Channel::Reliable => 0,
            Channel::Unreliable => 1,
        }
    }

    fn config() -> Vec<ChannelConfig> {
        let reliable_channel = ChannelConfig::Reliable(ReliableChannelConfig {
            channel_id: Channel::Reliable.id(),
            ..Default::default()
        });

        let unreliable_channel = ChannelConfig::Unreliable(UnreliableChannelConfig {
            channel_id: Channel::Unreliable.id(),
            ..Default::default()
        });

        vec![reliable_channel, unreliable_channel]
    }
}

#[cfg(test)]
mod tests {
    use bevy_renet::{
        renet::{RenetClient, RenetServer},
        RenetClientPlugin, RenetServerPlugin,
    };

    use super::*;
    use crate::core::network::{client::ConnectionSettings, server::ServerSettings};

    /// Preset for quickly testing networking
    #[derive(Clone, Copy)]
    pub(super) enum NetworkPreset {
        Server,
        Client,
        ServerAndClient { connected: bool },
    }

    /// Automates server and / or client creation for unit tests
    pub(super) struct TestNetworkPlugin {
        server: bool,
        client: bool,
        connected: bool,
    }

    impl Plugin for TestNetworkPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(MinimalPlugins);

            if self.server {
                let server_settings = ServerSettings {
                    port: 0,
                    ..Default::default()
                };

                app.insert_resource(
                    server_settings
                        .create_server()
                        .unwrap_or_else(|error| panic!("Unable to create server: {}", error)),
                )
                .add_plugin(RenetServerPlugin);
            }

            if self.client {
                let connection_settings = ConnectionSettings {
                    port: if self.server {
                        app.world.resource::<RenetServer>().addr().port()
                    } else {
                        0
                    },
                    ..Default::default()
                };

                app.insert_resource(
                    connection_settings
                        .create_client()
                        .unwrap_or_else(|error| panic!("Unable to create client: {}", error)),
                )
                .add_plugin(RenetClientPlugin);
            }

            if self.connected {
                app.update();
                app.update();
                app.update();
                assert!(
                    app.world.resource::<RenetClient>().is_connected(),
                    "Client should be connected"
                );
            }
        }
    }

    impl TestNetworkPlugin {
        pub(super) fn new(preset: NetworkPreset) -> Self {
            // Split into booleans for easily handling the logic
            match preset {
                NetworkPreset::Server => Self {
                    server: true,
                    client: false,
                    connected: false,
                },
                NetworkPreset::Client => Self {
                    server: false,
                    client: true,
                    connected: false,
                },
                NetworkPreset::ServerAndClient { connected } => Self {
                    server: true,
                    client: true,
                    connected,
                },
            }
        }
    }
}

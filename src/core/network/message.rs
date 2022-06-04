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
use bevy_renet::renet::{RenetClient, RenetServer};
use serde::{Deserialize, Serialize};

use super::{Channel, NetworkingState, SERVER_ID};

pub(super) struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ServerMessage>()
            .add_event::<ClientMessage>()
            .add_event::<MessageReceived>()
            .add_event::<MessageSent>()
            .add_system_set(
                SystemSet::on_update(NetworkingState::Connected)
                    .with_system(MessagePlugin::receive_server_message_system),
            )
            .add_system_set(
                SystemSet::on_update(NetworkingState::Connected)
                    .with_system(MessagePlugin::send_client_message_system),
            )
            .add_system_set(
                SystemSet::on_update(NetworkingState::Hosting)
                    .with_system(MessagePlugin::local_client_message_system),
            )
            .add_system_set(
                SystemSet::on_update(NetworkingState::Hosting)
                    .with_system(MessagePlugin::receive_client_message_system),
            )
            .add_system_set(
                SystemSet::on_update(NetworkingState::Hosting)
                    .with_system(MessagePlugin::send_server_message_system),
            );
    }
}

impl MessagePlugin {
    fn receive_server_message_system(
        mut server_events: EventWriter<ServerMessage>,
        mut client: ResMut<RenetClient>,
    ) {
        while let Some(message) = client.receive_message(Channel::Reliable.id()) {
            match bincode::deserialize(&message) {
                Ok(message) => server_events.send(message),
                Err(error) => {
                    error!(
                        "Unable to deserialize message from server: {}",
                        error.to_string()
                    );
                }
            };
        }
    }

    fn send_client_message_system(
        mut client_events: EventReader<ClientMessage>,
        mut client: ResMut<RenetClient>,
    ) {
        for message in client_events.iter() {
            match bincode::serialize(&message) {
                Ok(message) => client.send_message(Channel::Reliable.id(), message),
                Err(error) => error!(
                    "Unable to serialize message for server: {}",
                    error.to_string()
                ),
            };
        }
    }

    fn local_client_message_system(
        mut client_events: EventReader<ClientMessage>,
        mut receive_events: EventWriter<MessageReceived>,
    ) {
        for message in client_events.iter().cloned() {
            receive_events.send(MessageReceived {
                client_id: 0,
                message,
            })
        }
    }

    fn receive_client_message_system(
        mut receive_events: EventWriter<MessageReceived>,
        mut server: ResMut<RenetServer>,
    ) {
        for client_id in server.clients_id().iter().copied() {
            while let Some(message) = server.receive_message(client_id, Channel::Reliable.id()) {
                match bincode::deserialize(&message) {
                    Ok(message) => receive_events.send(MessageReceived { client_id, message }),
                    Err(error) => {
                        error!(
                            "Unable to deserialize message from client {}: {}",
                            client_id,
                            error.to_string()
                        );
                    }
                };
            }
        }
    }

    fn send_server_message_system(
        mut send_events: EventReader<MessageSent>,
        mut server_events: EventWriter<ServerMessage>,
        mut server: ResMut<RenetServer>,
    ) {
        for event in send_events.iter() {
            let message = match bincode::serialize(&event.message) {
                Ok(message) => message,
                Err(error) => {
                    error!(
                        "Unable serialize message for client(s): {}",
                        error.to_string()
                    );
                    continue;
                }
            };

            match event.kind {
                SendKind::Broadcast => {
                    server.broadcast_message(Channel::Reliable.id(), message);
                    server_events.send(event.message.clone());
                }
                SendKind::BroadcastExcept(client_id) => {
                    if client_id == SERVER_ID {
                        server.broadcast_message(Channel::Reliable.id(), message);
                    } else {
                        server.broadcast_message_except(client_id, Channel::Reliable.id(), message);
                        server_events.send(event.message.clone());
                    }
                }
                SendKind::Direct(client_id) => {
                    if client_id == SERVER_ID {
                        server_events.send(event.message.clone());
                    } else {
                        server.send_message(client_id, Channel::Reliable.id(), message);
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) struct MessageReceived {
    pub(crate) client_id: u64,
    pub(crate) message: ClientMessage,
}

#[allow(dead_code)]
pub(crate) struct MessageSent {
    pub(crate) kind: SendKind,
    pub(crate) message: ServerMessage,
}

#[allow(dead_code)]
pub(crate) enum SendKind {
    Broadcast,
    BroadcastExcept(u64),
    Direct(u64),
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum ServerMessage {
    ChatMessage { sender_id: u64, message: String },
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum ClientMessage {
    ChatMessage(String),
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use bevy_renet::RenetServerPlugin;

    use super::*;
    use crate::{
        core::network::server::ServerSettings,
        test_utils::{ConnectionPlugin, AVAILABLE_PORT},
    };

    #[test]
    fn client_messages() {
        let mut app = App::new();
        app.add_plugin(ConnectionPlugin)
            .add_plugin(TestMessagePlugin);

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Connected).unwrap();

        let mut client_events = app.world.resource_mut::<Events<ClientMessage>>();
        let client_message = ClientMessage::ChatMessage("Hi from client".into());
        client_events.send(client_message.clone());

        app.update();

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Hosting).unwrap();

        app.update();

        let mut receive_events = app.world.resource_mut::<Events<MessageReceived>>();
        let event = receive_events
            .drain()
            .next()
            .expect("The server should recieve a message from the client");

        assert_eq!(
            event.message, client_message,
            "The received message should match the one sent"
        );

        let client = app.world.resource::<RenetClient>();
        assert_eq!(
            event.client_id,
            client.client_id(),
            "Client ID from the event should match with the sender"
        );
    }

    #[test]
    fn local_client_messages() {
        let server_settings = ServerSettings {
            port: AVAILABLE_PORT
                .lock()
                .next()
                .expect("No available empty ports left"),
            ..Default::default()
        };

        let mut app = App::new();
        app.insert_resource(
            server_settings
                .create_server()
                .expect("Unable to create server from settings"),
        )
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin)
        .add_plugin(TestMessagePlugin);

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Hosting).unwrap();

        let mut client_events = app.world.resource_mut::<Events<ClientMessage>>();
        let client_message = ClientMessage::ChatMessage("Hi from local client".into());
        client_events.send(client_message.clone());

        app.update();

        let mut receive_events = app.world.resource_mut::<Events<MessageReceived>>();
        let event = receive_events
            .drain()
            .next()
            .expect("The server should recieve a message from the client");

        assert_eq!(
            event.message, client_message,
            "The received message should match the one sent"
        );

        assert_eq!(
            event.client_id, SERVER_ID,
            "Client ID from the event should match with the sender"
        );
    }

    // TODO: Extend tests once this PR will me merged:
    // https://github.com/lucaspoffo/renet/pull/17
    #[test]
    fn server_messages() {
        let mut app = App::new();
        app.add_plugin(ConnectionPlugin)
            .add_plugin(TestMessagePlugin);

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Hosting).unwrap();

        let server_message = ServerMessage::ChatMessage {
            sender_id: SERVER_ID,
            message: "Hi all from server".to_string(),
        };
        let mut send_events = app.world.resource_mut::<Events<MessageSent>>();
        send_events.send(MessageSent {
            kind: SendKind::Broadcast,
            message: server_message.clone(),
        });

        app.update();

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Connected).unwrap();

        app.update();

        let mut server_events = app.world.resource_mut::<Events<ServerMessage>>();
        let mut events_iter = server_events.drain();
        let first_event = events_iter
            .next()
            .expect("The local client should recieve a message from the server");

        let second_event = events_iter
            .next()
            .expect("The connected client should recieve a message from the server");

        assert_eq!(
            first_event, second_event,
            "The event for the local client and for the connected one should match",
        );
        assert!(
            events_iter.next().is_none(),
            "There shouldn't be more events"
        );

        assert_eq!(
            first_event, server_message,
            "The received message should match the one sent",
        );
    }

    struct TestMessagePlugin;

    impl Plugin for TestMessagePlugin {
        fn build(&self, app: &mut App) {
            app.add_state(NetworkingState::NoSocket)
                .add_plugin(MessagePlugin);
        }
    }
}

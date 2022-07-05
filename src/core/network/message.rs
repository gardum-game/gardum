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
use bevy_renet::renet::{RenetClient, RenetServer};
use iyes_loopless::prelude::*;
use serde::{Deserialize, Serialize};

use super::{client, Channel, SERVER_ID};

/// Contains systems that send and recieve reliable messages over the network.
/// Sending and receiving is done through events:
/// * Server receives a [`MessageReceived`] event when a message is received and emits a [`MessageSent`] event to send.
/// * Client receives a [`ServerMessage`] event when a message is received and emits a [`ClientMessage`] event to send.
pub(super) struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ServerMessage>()
            .add_event::<ClientMessage>()
            .add_event::<MessageReceived>()
            .add_event::<MessageSent>()
            .add_system(MessagePlugin::receive_server_message_system.run_if(client::connected))
            .add_system(MessagePlugin::send_client_message_system.run_if(client::connected))
            .add_system(
                MessagePlugin::local_client_message_system.run_if_resource_exists::<RenetServer>(),
            )
            .add_system(
                MessagePlugin::receive_client_message_system
                    .run_if_resource_exists::<RenetServer>(),
            )
            .add_system(
                MessagePlugin::send_server_message_system.run_if_resource_exists::<RenetServer>(),
            );
    }
}

impl MessagePlugin {
    fn receive_server_message_system(
        mut server_events: EventWriter<ServerMessage>,
        mut client: ResMut<RenetClient>,
    ) {
        while let Some(message) = client.receive_message(Channel::Reliable.id()) {
            match rmp_serde::from_slice(&message) {
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
            match rmp_serde::to_vec(&message) {
                Ok(message) => client.send_message(Channel::Reliable.id(), message),
                Err(error) => error!(
                    "Unable to serialize message for server: {}",
                    error.to_string()
                ),
            };
        }
    }

    /// Transforms [`ClientMessage`] events into [`MessageReceived`] events to "emulate"
    /// message sending for the listen server mode (when server is also a client)
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
                match rmp_serde::from_slice(&message) {
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
            let message = match rmp_serde::to_vec(&event.message) {
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

/// An event indicating that the message from client was received.
/// Emited only on server.
pub(crate) struct MessageReceived {
    pub(crate) client_id: u64,
    pub(crate) message: ClientMessage,
}

/// An event indicating that a server message has been sent.
/// This event should be used instead of sending messages directly.
/// Emited only on server.
pub(crate) struct MessageSent {
    pub(crate) kind: SendKind,
    pub(crate) message: ServerMessage,
}

/// Type of server message sending.
#[derive(Clone, Copy)]
pub(crate) enum SendKind {
    #[allow(dead_code)]
    Broadcast,
    BroadcastExcept(u64),
    #[allow(dead_code)]
    Direct(u64),
}

/// A message from server.
/// On client it represented in form of events of this struct.
/// On server this struct is a part of [`MessageSent`] event that
/// also contains information about client recipients.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum ServerMessage {
    ChatMessage { sender_id: u64, message: String },
}

/// A message from client.
/// On client it represented in form of events of this struct.
/// On server this struct is a part of [`MessageReceived`] event that
/// also contains information about sender client.
#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum ClientMessage {
    ChatMessage(String),
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use super::*;
    use crate::core::network::tests::{NetworkPreset, TestNetworkPlugin};

    #[test]
    fn client_messages() {
        let mut app = App::new();
        app.add_plugin(MessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        let mut client_events = app.world.resource_mut::<Events<ClientMessage>>();
        let client_message = ClientMessage::ChatMessage("Hi from client".into());
        client_events.send(client_message.clone());

        app.update();
        app.update();

        let client_id = app.world.resource::<RenetClient>().client_id();
        let mut receive_events = app.world.resource_mut::<Events<MessageReceived>>();
        let mut receive_events = receive_events.drain();

        let local_event = receive_events
            .next()
            .expect("The server should emit a local message for listen server mode");
        let remote_event = receive_events
            .next()
            .expect("The server should recieve a message from the client");
        assert!(
            receive_events.next().is_none(),
            "The server should be only two events"
        );

        assert_eq!(
            local_event.message, client_message,
            "The received message should match the one sent"
        );
        assert_eq!(
            local_event.client_id, SERVER_ID,
            "Client ID should should match the server ID for local message"
        );

        assert_eq!(
            remote_event.message, client_message,
            "The received message should match the one sent"
        );
        assert_eq!(
            remote_event.client_id, client_id,
            "Client ID from the remote event should match with the sender"
        );
    }

    #[test]
    fn local_client_messages() {
        let mut app = App::new();
        app.add_plugin(MessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::Server));

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

    #[test]
    fn server_messages() {
        let mut app = App::new();
        app.add_plugin(MessagePlugin)
            .add_plugin(TestNetworkPlugin::new(NetworkPreset::ServerAndClient {
                connected: true,
            }));

        let client_id = app.world.resource::<RenetClient>().client_id();
        for send_kind in [
            SendKind::Broadcast,
            SendKind::Direct(SERVER_ID),
            SendKind::Direct(client_id),
            SendKind::BroadcastExcept(SERVER_ID),
            SendKind::BroadcastExcept(client_id),
        ] {
            let chat_message = ServerMessage::ChatMessage {
                sender_id: SERVER_ID,
                message: "Hello from server".to_string(),
            };
            let mut send_events = app.world.resource_mut::<Events<MessageSent>>();
            send_events.send(MessageSent {
                kind: send_kind,
                message: chat_message.clone(),
            });

            app.update();
            app.update();

            let mut server_events = app.world.resource_mut::<Events<ServerMessage>>();
            let mut events_iter = server_events.drain();
            let received_message = events_iter
                .next()
                .expect("Message from server should be received");

            assert_eq!(
                received_message, chat_message,
                "The received message should match the one sent",
            );

            if let SendKind::Broadcast = send_kind {
                let duplciated_message = events_iter
                    .next()
                    .expect("Second message should be additonaly duplicated for local client (for listen server mode)");

                assert_eq!(
                    received_message, duplciated_message,
                    "The event for the local client and for the connected one should match",
                );
            }

            assert!(
                events_iter.next().is_none(),
                "There shouldn't be more events"
            );
        }
    }
}

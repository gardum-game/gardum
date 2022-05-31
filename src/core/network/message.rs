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

use super::{Channels, NetworkingState};

pub(super) struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ServerMessage>()
            .add_event::<ClientMessageWithId>()
            .add_system_set(
                SystemSet::on_update(NetworkingState::Hosting)
                    .with_system(MessagePlugin::client_events_reading_system),
            )
            .add_system_set(
                SystemSet::on_update(NetworkingState::Connected)
                    .with_system(MessagePlugin::server_events_reading_system),
            );
    }
}

impl MessagePlugin {
    fn client_events_reading_system(
        mut message_events: EventWriter<ClientMessageWithId>,
        mut server: ResMut<RenetServer>,
    ) {
        for client_id in server.clients_id().iter().copied() {
            while let Some(message) = server.receive_message(client_id, Channels::Reliable.id()) {
                match bincode::deserialize(&message) {
                    Ok(message) => message_events.send(ClientMessageWithId { client_id, message }),
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

    fn server_events_reading_system(
        mut message_events: EventWriter<ServerMessage>,
        mut client: ResMut<RenetClient>,
    ) {
        while let Some(message) = client.receive_message(Channels::Reliable.id()) {
            match bincode::deserialize(&message) {
                Ok(message) => message_events.send(message),
                Err(error) => {
                    error!(
                        "Unable to deserialize message from server: {}",
                        error.to_string()
                    );
                }
            };
        }
    }
}

#[allow(dead_code)]
struct ClientMessageWithId {
    client_id: u64,
    message: ClientMessage,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum ServerMessage {
    ChatMessage { sender_id: u64, message: String },
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum ClientMessage {
    ChatMessage(String),
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use bevy_renet::{RenetClientPlugin, RenetServerPlugin};

    use super::*;
    use crate::{
        core::network::{client::ConnectionSettings, server::ServerSettings},
        test_utils::AVAILABLE_PORT,
    };

    #[test]
    fn sending_and_receiving_messages() {
        let server_settings = ServerSettings {
            port: AVAILABLE_PORT.lock().next().unwrap(),
            ..Default::default()
        };
        let connection_settings = ConnectionSettings {
            port: server_settings.port,
            ..Default::default()
        };

        let mut app = App::new();
        app.add_plugin(TestMessagePlugin)
            .add_plugins(MinimalPlugins)
            .add_plugin(RenetServerPlugin)
            .add_plugin(RenetClientPlugin)
            .insert_resource(
                server_settings
                    .create_server()
                    .expect("Server should be created succesfully from settings"),
            )
            .insert_resource(
                connection_settings
                    .create_client()
                    .expect("Client should be created succesfully from settings"),
            );

        app.update();
        app.update();
        app.update();

        let mut client = app.world.resource_mut::<RenetClient>();
        assert!(
            client.is_connected(),
            "The client must be connected to the server to send messages",
        );

        let client_message = ClientMessage::ChatMessage("Hi from client".into());
        client.send_message(
            Channels::Reliable.id(),
            bincode::serialize(&client_message).expect("Unable to serialize client message"),
        );

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Hosting).unwrap();

        app.update();
        app.update();

        let mut message_events = app.world.resource_mut::<Events<ClientMessageWithId>>();
        let event = message_events
            .drain()
            .next()
            .expect("The server should recieve a message from the client");

        assert_eq!(
            event.message, client_message,
            "The received message should match the one sent"
        );

        let mut server = app.world.resource_mut::<RenetServer>();
        let server_message = ServerMessage::ChatMessage {
            sender_id: event.client_id,
            message: "Hi from server".into(),
        };
        server.send_message(
            event.client_id,
            Channels::Reliable.id(),
            bincode::serialize(&server_message).expect("Unable to serialize server message"),
        );

        let mut networking_state = app.world.resource_mut::<State<NetworkingState>>();
        networking_state.set(NetworkingState::Connected).unwrap();

        app.update();
        app.update();

        let mut message_events = app.world.resource_mut::<Events<ServerMessage>>();
        let event = message_events
            .drain()
            .next()
            .expect("The client should recieve a message from the server");

        assert_eq!(
            event, server_message,
            "The received message should match the one sent"
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

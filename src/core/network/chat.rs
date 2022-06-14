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
use iyes_loopless::prelude::*;

use super::{
    message::{ClientMessage, MessageReceived, MessageSent, SendKind, ServerMessage},
    NetworkingState,
};

pub(super) struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::broadcast_messages_system.run_in_state(NetworkingState::Hosting));
    }
}

impl ChatPlugin {
    fn broadcast_messages_system(
        mut receive_events: EventReader<MessageReceived>,
        mut send_events: EventWriter<MessageSent>,
    ) {
        for event in receive_events.iter() {
            let ClientMessage::ChatMessage(message) = &event.message;
            send_events.send(MessageSent {
                kind: SendKind::BroadcastExcept(event.client_id),
                message: ServerMessage::ChatMessage {
                    sender_id: event.client_id,
                    message: message.clone(),
                },
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use super::*;
    use crate::core::network::SERVER_ID;

    #[test]
    fn messages_broadcasted() {
        let mut app = App::new();
        app.add_plugin(TestChatPlugin);

        const CHAT_MESSAGE: &str = "Player message";
        let mut receive_events = app.world.resource_mut::<Events<MessageReceived>>();
        receive_events.send(MessageReceived {
            client_id: SERVER_ID,
            message: ClientMessage::ChatMessage(CHAT_MESSAGE.to_string()),
        });

        app.update();

        let mut send_events = app.world.resource_mut::<Events<MessageSent>>();
        let sent_message = send_events
            .drain()
            .next()
            .expect("A message should be sent to other clients");

        assert!(
            matches!(sent_message.kind, SendKind::BroadcastExcept(client_id) if client_id == SERVER_ID),
            "The sent message should be broadcast to everyone except the sender"
        );

        let ServerMessage::ChatMessage { sender_id, message } = sent_message.message;
        assert_eq!(
            sender_id, SERVER_ID,
            "Chat message should contain the same sender id as the received message"
        );
        assert_eq!(
            message, CHAT_MESSAGE,
            "The message sent should match the received chat message"
        );
    }

    struct TestChatPlugin;

    impl Plugin for TestChatPlugin {
        fn build(&self, app: &mut App) {
            app.add_loopless_state(NetworkingState::Hosting)
                .add_event::<MessageReceived>()
                .add_event::<MessageSent>()
                .add_plugin(ChatPlugin);
        }
    }
}

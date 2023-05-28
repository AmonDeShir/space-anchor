//Based on https://github.com/AmonDeShir/bevy-ws-server/blob/main/src/lib.rs

use async_channel::TryRecvError;
use async_tungstenite::tungstenite::Message;
use bevy::prelude::*;

use crate::{Server, UnparsedMessages, ClientConnected, ClientDisconnected};

/// Starts the server listening.
pub fn startup(listener: Res<Server>) {
    listener.listen("127.0.0.1:2137");
}

/// This system handles messages sended by client, convert them to string and pack them in the UnparsedMessages resource.
/// This system also handles client dissconnection, by invoke the ClientDisconnected event
pub fn handle_server_events(mut server: ResMut<Server>, mut messages: ResMut<UnparsedMessages>, mut ev_server: EventWriter<ClientDisconnected>) {
    let mut removed = vec![];
    messages.0.clear();
    
    for (id, connection) in server.connections.iter() {
        loop {
            match connection.receiver.try_recv() {
                Ok(message) => {
                    if let Message::Text(text) = message {
                        messages.0.push((*id, text));
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => {
                    removed.push(*id);
                    break;
                }
            }
        }
    }

    for id in removed.iter() {
        server.connections.remove(&id);
        ev_server.send(ClientDisconnected(*id));
    }

}


/// Handles a new client. Generates an ID for it and moves it from the acceptQueue to the connections hash map.
/// This system also invokes the ClientDisconnected event
pub fn handle_accept_queue(mut server: ResMut<Server>, mut ev_server: EventWriter<ClientConnected>) {
    for id in  server.handle_accept_queue() {
        ev_server.send(ClientConnected(id));
    }
}

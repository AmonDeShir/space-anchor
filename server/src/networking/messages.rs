use bevy_renet::renet::RenetServer;
use serde::{Serialize, Deserialize};
use serde_json;

use super::{SendError, channel::Channel};

#[derive(Debug, Serialize, Deserialize)]
/// This enum contains message types that can be sent to clients. It's designed for sending small messages
/// when message ordering is crucial and when messages must be delivered. 
/// 
/// This enum uses renet reliable channel.
pub enum ServerMessage {
    RequestConnect(),
    PlayerJoined(String),
    ConnectionStatus(),
    PONG(),
}

#[derive(Serialize, Deserialize)]
/// This enum contains message types that can be sent to clients. It's designed for sending big messages, like map data,
/// that aren't frequent. It can be used when the message is large and must be delivered. 
/// 
/// The message is sliced into multiple chunks so that it can be sent in multiple frames instead of sending all of it in one packet.
///
/// This enum uses renet chunk channel.
pub enum ServerChunkMessage {

}

#[derive(Serialize, Deserialize)]
/// This enum contains message types that can be sent to clients. It's designed for sending small pieces of data frequently.
/// This channel is unreliable, so there is a small chance that messages will not be received at all. 
/// There are no guarantees about the order in which clients will receive messages.
/// 
/// This enum uses renet unrealiable channel.
pub enum ServerFrequentMessage {

}

#[derive(Debug, Serialize, Deserialize)]
/// Channel for all kinds of messages that can be received from clients.
pub enum ClientMessage {
    Connect(String),
    PING(),
}

impl ServerChunkMessage {
    /// Sends a message directly to one client. Returns Err(SendError::CannotSend) if the message cannot be sent to the client.
    pub fn send(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        if !server.can_send_message(id, Channel::Chunk.id()) {
            
            let message = serde_json::to_string(self).unwrap();
            server.send_message(id, Channel::Chunk.id(), message);

            Ok(())
        }

        else {
            Err(SendError::CannotSend)
        }
    }

    /// Sends a message to all clients, but first checks if the message can be sent to all clients. If not, the message will not be sent to any client, and the function will return Err(SendError::CannotSend).
    pub fn broadcast(&self, server: &mut RenetServer) -> Result<(), SendError> {
        for id in server.clients_id() {
            if !server.can_send_message(id, Channel::Chunk.id()) {
                return Err(SendError::CannotSend);
            }
        }

        let message = serde_json::to_string(self).unwrap();
        server.broadcast_message(Channel::Chunk.id(), message.as_bytes().to_vec());

        Ok(())
    }

    /// Sends a message to all clients except one, but first checks if the message can be sent to all clients. If not, the message will not be sent to any client, and the function will return Err(SendError::CannotSend).
    pub fn broadcast_except(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        for all_id in server.clients_id() {
            if all_id == id {
                continue;
            }

            if !server.can_send_message(id, Channel::Chunk.id()) {
                return Err(SendError::CannotSend);
            }
        }

        let message = serde_json::to_string(self).unwrap();
        server.broadcast_message_except(id, Channel::Chunk.id(), message);

        Ok(())
    }
}

impl ServerMessage {
    /// Sends a message directly to one client. Returns Err(SendError::CannotSend) if the message cannot be sent to the client.
    pub fn send(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        if !server.can_send_message(id, Channel::Reliable.id()) {
            return Err(SendError::CannotSend);
        }
        
        let message = serde_json::to_string(self).unwrap();
        server.send_message(id, Channel::Reliable.id(), message);

        Ok(())
    }

    /// Sends a message to all clients, but first checks if the message can be sent to all clients. If not, the message will not be sent to any client, and the function will return Err(SendError::CannotSend).
    pub fn broadcast(&self, server: &mut RenetServer) -> Result<(), SendError> {
        for id in server.clients_id() {
            if !server.can_send_message(id, Channel::Reliable.id()) {
                return Err(SendError::CannotSend);
            }
        }

        let message = serde_json::to_string(self).unwrap();
        server.broadcast_message(Channel::Reliable.id(), message);

        Ok(())
    }

    /// Sends a message to all clients except one, but first checks if the message can be sent to all clients. If not, the message will not be sent to any client, and the function will return Err(SendError::CannotSend).
    pub fn broadcast_except(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        for id in server.clients_id() {
            if !server.can_send_message(id, Channel::Reliable.id()) {
                return Err(SendError::CannotSend);
            }
        }

        let message = serde_json::to_string(self).unwrap();
        server.broadcast_message_except(id, Channel::Reliable.id(), message);
        
        Ok(())
    }
}
use bevy_renet::renet::RenetServer;
use serde::{Serialize, Deserialize};
use serde_json;

use super::{SendError, channel::Channel};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    RequestConnect(),
    PlayerJoined(String),
    PONG(),
}

#[derive(Serialize, Deserialize)]
pub enum ServerChunkMessage {

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Connect(String),
    PING(),
}

impl ServerChunkMessage {
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
    pub fn send(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        if !server.can_send_message(id, Channel::Reliable.id()) {
            return Err(SendError::CannotSend);
        }
        
        let message = serde_json::to_string(self).unwrap();
        server.send_message(id, Channel::Reliable.id(), message);

        Ok(())
    }

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
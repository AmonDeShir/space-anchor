use bevy_renet::renet::RenetServer;
use bevy::prelude::*;
pub use messages_derive::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use core::fmt::Debug;

/// A Bevy resource that contains all messages received from clients as strings, paired with the corresponding client ID. This resource should be created once and used by the whole application.
#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct UnparsedMessages(pub Vec<(u64, String)>);


/// A Bevy resource that contains all messages that fit the particular ReceivableMessage instance. Any plugin that can receive messages from clients should have an instance of this resource.
#[derive(Debug, Default, Resource)]
pub struct ParsedMessages<T: ReceivableMessage>(pub Vec<(u64, T)>);

/// Trait with methods for parsing and handling client messages.
pub trait ReceivableMessage : DeserializeOwned + Debug + Send + Sync + Clone + PartialEq + 'static {    
    /// Parse string messages into ParsedMessages<ReceivableMessage> enum.
    fn parse(unparsed_messages: Res<UnparsedMessages>, mut messages: ResMut<ParsedMessages<Self>>) {
        messages.0.clear();
        
        for (user, message) in unparsed_messages.iter() {
            if let Ok(message) = serde_json::from_str::<Self>(message) {
                messages.0.push((*user, message.clone()));
            }
        }
    }    

    /// Search for particular message in the ParsedMessages resource and if success return it as tuple. The first tuple argument is a client id.
    /// works only with enum variants without parameter, for parameter variants use the generate_finder macro.
    fn find<'w, T: ReceivableMessage>(&'w self, messages: Res<'w, ParsedMessages<T>>) -> Option<(u64, T)> 
        where Self: PartialEq<T> {
        
        for (id, data) in messages.0.iter() {        
            if self == data {
                return Some((*id, data.clone()));
            }
        }
  
        return None;
    }
}

/// Macro that searches for a particular message in the ParsedMessages resource and if success returns it as a tuple. The first tuple argument is a client id.
/// works only with enum variants with parameter, for non-parameter variants use the ReceivableMessage::find method.
#[macro_export]
macro_rules! generate_finder {
    ($name:path, $messages:expr) => {
        $messages.0.iter().find_map(|(user, data)| {
            match data {
                $name(data) => Some((user, data)),
                _ => None,
            }
        })
    };
}


#[derive(Debug)]
pub enum SendError {
    CannotSend
}

/// Trait with methods for sending messages to client.
pub trait SendableMessage: Serialize {
    /// Returns the id of the channel that the instance of this trait will be use to send messages
    fn get_channel_id() -> u8;

    /// Sends a message to client, but first checks if the message can be sent. If not, the message will not be sent and the function will return Err(SendError::CannotSend).
    fn send(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        if server.can_send_message(id, Self::get_channel_id()) {
            
            let message = serde_json::to_string(self).unwrap();
            server.send_message(id, Self::get_channel_id(), message);

            Ok(())
        }

        else {
            Err(SendError::CannotSend)
        }
    }

    /// Sends a message to all clients, but first checks if the message can be sent to all clients. If not, the message will not be sent to any client, and the function will return Err(SendError::CannotSend).
    fn broadcast(&self, server: &mut RenetServer) -> Result<(), SendError> {
        for id in server.clients_id() {
            if !server.can_send_message(id, Self::get_channel_id()) {
                return Err(SendError::CannotSend);
            }
        }

        let message = serde_json::to_string(self).unwrap();
        server.broadcast_message(Self::get_channel_id(), message.as_bytes().to_vec());

        Ok(())
    }

    /// Sends a message to all clients except one, but first checks if the message can be sent to all clients. If not, the message will not be sent to any client, and the function will return Err(SendError::CannotSend).
    fn broadcast_except(&self, server: &mut RenetServer, id: u64) -> Result<(), SendError> {
        for all_id in server.clients_id() {
            if all_id == id {
                continue;
            }

            if !server.can_send_message(id, Self::get_channel_id()) {
                return Err(SendError::CannotSend);
            }
        }

        let message = serde_json::to_string(self).unwrap();
        server.broadcast_message_except(id, Self::get_channel_id(), message);

        Ok(())
    }   
}
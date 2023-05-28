use bevy::prelude::*;
pub use bevy_websocket_server_derive::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use core::fmt::Debug;

use crate::Server;

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
    /// Sends a message to client
    fn send(&self, server: &mut Server, id: &u64) {
        let message = serde_json::to_string(self).unwrap();
        server.send(&id, message);
    }

    /// Try sends a message to all clients
    fn broadcast(&self, server: &mut Server) {
        let message: String = serde_json::to_string(self).unwrap();
        server.broadcast(message);
    }

    /// Sends a message to all clients except one
    fn broadcast_except(&self, server: &mut Server, id: &u64) {
        let message: String = serde_json::to_string(self).unwrap();
        server.broadcast_except(id, message);
    }   
}
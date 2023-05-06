use bevy::prelude::Resource;
use serde::{Serialize, Deserialize};
use messages::{SendableMessage, SendableChunkMessage, SendableFrequentMessage, ReceivableMessage};
use channels::Channel;

/// All kinds of messages that can be sended to clients.
/// Uses the reliable channel
#[derive(Debug, Serialize, SendableMessage)]
pub enum ServerMessage {
    /// Message sended to client after successful rentet connection. 
    /// It informs client that server is ready and is waiting for login or registration.
    RequestConnect,

    /// Empty message. Should be sent to client when requested by ClientMessage::PING message,
    /// to prevent auto client disconnection after 10 minutes of client inactivity.
    PONG,
}


/// All kinds of messages that can be sended to clients.
/// Uses the chunk channel
#[derive(Serialize, SendableChunkMessage)]
pub enum ServerChunkMessage {}

/// All kinds of messages that can be sended to clients.
/// Uses the unreliable channel
#[derive(Serialize, SendableFrequentMessage)]
pub enum ServerFrequentMessage {}

/// All kinds of messages that can be received from clients.
#[derive(Debug, Clone, Deserialize, PartialEq, Resource, ReceivableMessage)]
pub enum ClientMessage {
    /// Empty message, request the PONG action. Should be sent by client if there wasn't any communication 
    /// since last 10 minutes to prevent auto disconnection.   
    PING,
    /// Default ClientMessage enum value, indicating that no message was sent by the client.
    NONE,
}
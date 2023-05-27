use bevy::prelude::Resource;
use bevy_websocket_server::{SendableMessage, ReceivableMessage};
use serde::{Serialize, Deserialize};

/// All information needed to create a new player.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerData {
    pub name: String,
    pub race: String,
}

/// All kinds of messages that can be sent to clients.
#[derive(Debug, Serialize, SendableMessage)]
pub enum ServerMessage {
    PlayerJoined(String),
    ConnectionSuccess,
    ConnectionFailed(String),
    RequestRegistration,
    RegistrationFailed(String),
}

/// All kinds of messages that can be received from clients.
#[derive(Debug, Clone, Deserialize, PartialEq, Resource, ReceivableMessage)]
pub enum ClientMessage {
    /// Login request, client will send this when it needs to log in, taking the name as a parameter.
    Connect(String),
    /// Register request, client will send this when the player wants to create a new character, taking the character data as a parameter.
    Register(PlayerData),

    /// Default ClientMessage enum value, indicating that no message was sent by the client.
    NONE,
}


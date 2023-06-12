use bevy::prelude::{Resource, Vec2};
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
    /// Message sent to all already connected clients when a new player joins.
    /// 
    /// # Arguments
    /// * `String` - Name of the player who just joined the game.
    ///
    PlayerJoined(String),
    
    /// Message sent to a client that starts the connecting process after successful login or registration.
    ConnectionSuccess,

    /// Message sent to a client that starts the connecting process when it failed.
    /// 
    /// # Arguments
    /// 
    /// * `String` - The reason why the connecting process has failed.
    /// 
    ConnectionFailed(String),

    /// Message sent to a client that starts the connecting process when the registration of a new character is necessary.
    RequestRegistration,
    
    /// Message sent to a client that starts the registration process when it failed.
    /// 
    /// # Arguments
    /// 
    /// * `String` - The reason why the registration process has failed.
    /// 
    RegistrationFailed(String),
    
    /// Message that contains the admiral's ID of a client. 
    ///     
    /// # Arguments
    /// 
    /// * `u64` - Admiral's ID
    /// 
    PlayerAdmiralId(u64),
}

/// All kinds of messages that can be received from clients.
#[derive(Debug, Clone, Deserialize, PartialEq, Resource, ReceivableMessage)]
pub enum ClientMessage {
    /// Login request. The client sends this when it needs to log in
    /// 
    /// ## Arguments
    /// 
    /// * `String` - The name of the player the client is trying to log in to.
    /// 
    Connect(String),

    /// Register request. The client sends this when the player wants to create a new character.
    /// 
    /// ## Arguments
    /// 
    /// * `PlayerData` - Information about creating character.
    /// 
    Register(PlayerData),

    /// Move request. The client sends this when the player clicks on the map in order to move a player.
    /// 
    /// ## Arguments
    /// 
    /// * `Vec2` - Point to which the player should be moved.
    /// 
    MoveTo(Vec2),
    
    /// Admiral Request. The client sends this after login to receive the admiral ID.
    GetPlayerAdmiralId(),

    /// Default ClientMessage enum value, indicating that no message was sent by the client.
    NONE,
}

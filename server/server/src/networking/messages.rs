use bevy_websocket_server::SendableMessage;
use serde::Serialize;

/// All kinds of messages that can be sended to clients.
#[derive(Debug, Serialize, SendableMessage)]
pub enum ServerMessage {
    /// Message sended to client after successful connection of a new client. 
    /// It informs client that server is ready and is waiting for login or registration.
    RequestConnect,

    /// Message sent to clients when a logged-in player disconnects.
    Disconnected,
}
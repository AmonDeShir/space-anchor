use bevy::prelude::{Vec2, Resource};
use bevy_websocket_server::{SendableMessage, ReceivableMessage};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
/// Packet containing all information about an admiral.
/// 
/// **Note:** Rotation is in radians.
pub struct AdmiralPacket {
    pub id: u64,
    pub name: String,
    pub fraction: String,
    pub race: String,
    pub pos: Vec2,
    pub rotation: f32,
}

/// All kinds of messages that can be sent to clients.
#[derive(Debug, Serialize, SendableMessage)]
pub enum ServerMessage {
    /// Response that will be sent to the client when it sights an admiral.
    /// It will also be sent when the client sends the GetAllAdmiralsInSign request.
    /// 
    /// ## Arguments
    /// 
    /// * `AdmiralPacket` - All data needed to display an admiral instance.
    /// 
    AdmiralAppears(AdmiralPacket),

    /// Response that will be sent to the client when it loses sight of an admiral.
    /// 
    /// ## Arguments
    /// 
    /// * `u64` - Id of the disappeared admiral.
    /// 
    AdmiralDisappears(u64),

    /// Response with information about the position of all admirals in sight of the client. 
    /// This response should be sent to the client every 100ms.
    /// 
    /// ## Arguments
    /// 
    /// Enum of a vector of enums:
    /// * `u64` - Admiral's id,
    /// * `Vec2` - Admiral's position,
    /// * `f32` - Admiral's rotation in radians,
    /// 
    /// and `u128` - Current server time in milliseconds.
    PositionOfAdmiralsInSign((Vec<(u64, Vec2, f32)>, u128)),
}

/// All kinds of messages that can be received from clients.
#[derive(Debug, Clone, Deserialize, PartialEq, Resource, ReceivableMessage)]
pub enum ClientMessage {
    /// This request will be sent by the client in order to synchronize its map state with the server.
    /// All clients that have just logged in should send it. It can also be sent when a significant lag occurs.
    GetAllAdmiralsInSign(),

    /// Default ClientMessage enum value, indicating that no message was sent by the client.
    NONE,
}

use bevy_renet::renet::{ReliableChannelConfig, UnreliableChannelConfig, ChunkChannelConfig};

/// Renet channels,  
pub enum Channel {
    /// Configuration for a reliable and ordered channel. Messages will be received in the order they were sent. If a message is lost it'll be resent.
    Reliable,
    /// Configuration for a unreliable and unordered channel. Messages sent in this channel can be lost and arrive in an different order that they were sent.
    Unreliable,
    /// Configuration for a chunk channel, used for sending big and reliable messages, that are not so frequent, level initialization as an example. 
    /// The message is sliced in multiple chunks so it can be sent in multiple frames, instead of sending all of it in one packet. One message in flight at a time.
    Chunk,
}

impl Channel {
    /// Returns the unique ID of the channel.
    pub fn id(&self) -> u8 {
        match self {
            Channel::Reliable => ReliableChannelConfig::default().channel_id,
            Channel::Unreliable => UnreliableChannelConfig::default().channel_id,
            Channel::Chunk => ChunkChannelConfig::default().channel_id,
        }
    }
}
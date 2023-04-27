use renet::{ReliableChannelConfig, UnreliableChannelConfig, ChunkChannelConfig};

pub enum Channel {
    Reliable,
    Unreliable,
    Chunk,
}

impl Channel {
    pub fn id(&self) -> u8 {
        match self {
            Channel::Reliable => ReliableChannelConfig::default().channel_id,
            Channel::Unreliable => UnreliableChannelConfig::default().channel_id,
            Channel::Chunk => ChunkChannelConfig::default().channel_id,
        }
    }
}
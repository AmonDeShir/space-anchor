
/// This is event is used when a client establishes a connection with the server. u64 represents client's id.
pub struct ClientConnected(pub u64);

/// This is event is used when a client connection is ended or lost. u64 represents client's id.
pub struct ClientDisconnected(pub u64);

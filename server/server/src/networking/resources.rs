use std::{net::UdpSocket, time::SystemTime};
use bevy_renet::{renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication}};
use crate::version;

/// Current version of the server api, Renet will use it to prevent connection with outdated clients.
const PROTOCOL_ID: u64 = version!();

/// Create new rent server, with default renet configuration.
pub fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:2137".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
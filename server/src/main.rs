use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{renet::{RenetServer, RenetConnectionConfig, ServerConfig, ServerAuthentication}, RenetServerPlugin};

const PROTOCOL_ID: u64 = 7;

fn main() {
    App::new()
        .add_plugin(AssetPlugin::default())        
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(new_renet_server())
        .run();
}

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:2137".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
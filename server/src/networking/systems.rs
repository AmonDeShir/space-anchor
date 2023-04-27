use bevy_renet::renet::{ServerEvent, RenetServer};
use bevy::prelude::*;

use crate::networking::ServerMessage;
use crate::player::ClientID;
use crate::readable::components::Name;

use super::{CurrentServerMessages, ClientMessage};
use super::channel::Channel;
use std::str;
use serde_json;

pub fn handle_server_connection(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &ClientID, &Name)>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("Client connected: {}", id);
                ServerMessage::RequestConnect().send(&mut server, *id).unwrap();
            }
            
            ServerEvent::ClientDisconnected(id) => {
                info!("Client disconnected: {}", id);

                for (player, ClientID(player_id), Name(player_name)) in players.iter() {
                    if player_id == id {
                        info!("Player {} disconnected", player_name);
                        commands.entity(player).despawn();
                        break;
                    }
                }
            }
        }
    }
}

pub fn server_recieve_messages(mut server: ResMut<RenetServer>, mut messages: ResMut<CurrentServerMessages>) {
    messages.clear();
    
    for client_id in server.clients_id().into_iter() {
        for channel in [Channel::Reliable, Channel::Unreliable] {
            while let Some(message) = server.receive_message(client_id, channel.id()) {

                let str = match str::from_utf8(&message) {
                    Ok(s) => s,
                    Err(e) => panic!("Message conversion error: {}", e),
                };
                
                info!("Recived: {}", str);
                let client_message = serde_json::from_str(&str).unwrap();
                messages.push((client_id, client_message));
                
            }
        }
    }
}

pub fn server_ping_pong(mut server: ResMut<RenetServer>, mut messages: Res<CurrentServerMessages>) {
    for (client, message) in messages.iter() {
        match message {
            ClientMessage::PING() => {
                ServerMessage::PONG().send(&mut server, *client).unwrap();
            },
            _ => {}
        }
    }
}
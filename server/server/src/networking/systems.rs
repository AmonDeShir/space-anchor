use bevy_renet::renet::{ServerEvent, RenetServer};
use bevy::prelude::*;

use crate::networking::ServerMessage;
use crate::player::ClientID;
use crate::readable::components::Name;

use super::messages::ClientMessage;
use std::str;
use channels::Channel;
use messages::{SendableMessage, ReceivableMessage, UnparsedMessages, ParsedMessages};

/// This system handles Renet events, which sends connection requests to new players and removes the ClientID component from disconnected ones.
pub fn handle_renet_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &ClientID, &Name)>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                info!("Client connected: {}", id);
                ServerMessage::RequestConnect.send(&mut server, *id).unwrap();
            }
            
            ServerEvent::ClientDisconnected(id) => {
                info!("Client disconnected: {}", id);

                for (player, ClientID(player_id), Name(player_name)) in players.iter() {
                    if player_id == id {
                        info!("Player {} disconnected", player_name);
                        commands.entity(player).remove::<ClientID>();
                        break;
                    }
                }
            }
        }
    }
}

/// This system handles renet messages sended by client, convert them to string and pack them in the UnparsedMessages resource.
pub fn handle_received_messages(mut server: ResMut<RenetServer>, mut messages: ResMut<UnparsedMessages>) {
    messages.clear();
    
    for client_id in server.clients_id().into_iter() {
        for channel in [Channel::Reliable, Channel::Unreliable] {
            while let Some(message) = server.receive_message(client_id, channel.id()) {

                let str = match str::from_utf8(&message) {
                    Ok(s) => s,
                    Err(e) => panic!("Message conversion error: {}", e),
                };
                
                info!("Recived: {}", str);
                messages.push((client_id, String::from(str)));                
            }
        }
    }
}

// This system handles ping requests from the client and sends pong responses to prevent client disconnection due to 10 minutes of inactivity.
pub fn handle_ping_request(mut server: ResMut<RenetServer>, messages: Res<ParsedMessages<ClientMessage>>) {
    if let Some((client, _)) = ClientMessage::PING.find(messages) {
        ServerMessage::PONG.send(&mut server, client).unwrap();
    }
}
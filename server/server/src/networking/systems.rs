use bevy::prelude::*;
use bevy_websocket_server::{Server, ClientConnected, ClientDisconnected, SendableMessage};

use crate::networking::ServerMessage;
use crate::player::ClientID;
use crate::readable::components::Name;

/// This system handles server events, it sends connection requests to new players and removes the ClientID component from disconnected ones.
pub fn handle_server_events(
    mut commands: Commands,
    mut server: ResMut<Server>,
    players: Query<(Entity, &ClientID, &Name)>,
    mut connected_ev: EventReader<ClientConnected>,
    mut disconnected_ev: EventReader<ClientDisconnected>,

) {
    for ClientConnected(id) in connected_ev.iter() {
        info!("Client connected: {}", id);
        ServerMessage::RequestConnect.send(&mut server, id);
    }

    for ClientDisconnected(id) in disconnected_ev.iter() {
        info!("Client disconnected: {}", id);

        for (player, ClientID(player_id), Name(player_name)) in players.iter() {
            if *player_id == *id {
                info!("Player {} disconnected", player_name);
                commands.entity(player).remove::<ClientID>();
                ServerMessage::Disconnected(player_name.clone()).broadcast_except(&mut server, id);
            }
        }
    }
}
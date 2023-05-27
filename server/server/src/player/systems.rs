use bevy::prelude::*;
use bevy_websocket_server::{Server, ParsedMessages, generate_finder, SendableMessage};

use crate::{readable::components::Name, admiral::AdmiralBundle};

use super::{messages::{ClientMessage, ServerMessage}, Player, ClientID};

/// This system handles the client login request.
///
/// It queries all already logged-in players (entities with Player and ClientID components) and returns a `ConnectionFailed` message 
/// if a player is already logged in with the requested name.
///
/// It also queries all unlogged-in players (entities with a Player component but without a ClientID component). 
/// If an entity with the requested name exists, it adds the `ClientID` component to it, sends a `ConnectionSuccess` message to the client, 
/// and broadcasts a `PlayerJoined` message to all other clients.
///
/// If a player with the requested name does not exist, it sends a `RequestRegistration` message to the client.
pub fn login(
    mut server: ResMut<Server>, 
    mut commands: Commands,
    messages: Res<ParsedMessages<ClientMessage>>,
    logged_players: Query<(&Player, &Name), With<ClientID>>,
    available_players: Query<(&Player, Entity, &Name), Without<ClientID>>
) {
    if let Some((user, player_name)) = generate_finder!(ClientMessage::Connect, messages) {
        if logged_players.into_iter().find(|(_, Name(name))| name == player_name).is_some() {
            ServerMessage::ConnectionFailed("Player already connected!".to_string()).send(&mut server, user);
            return;
        }

        match available_players.into_iter().find(|(_, _, Name(name))| name == player_name) {
            Some((_, entity, _)) => {
                commands.entity(entity).insert(ClientID(*user));

                ServerMessage::PlayerJoined(player_name.clone()).broadcast_except(&mut server, user);                
                ServerMessage::ConnectionSuccess.send(&mut server, user);
            },
            None => {
                ServerMessage::RequestRegistration.send(&mut server, user);
            }
        }
    }
}

/// This system handles the client registration request. 
/// 
/// It searches for players with the same name and if found, it returns a RegistrationFailed message to the client. 
/// If the name is unique, it creates a new player entity and broadcasts a PlayerJoined message to all clients except the new one. 
/// Finally, it sends a ConnectionSuccess message to the new client to confirm successful registration.
pub fn register(
    mut server: ResMut<Server>, 
    mut commands: Commands,
    messages: Res<ParsedMessages<ClientMessage>>,
    players: Query<(&Player, &Name)>
) {
    if let Some((user, data)) = generate_finder!(ClientMessage::Register, messages) {
        if players.into_iter().find(|(_, Name(name))| name == &data.name).is_some() {
            ServerMessage::RegistrationFailed("Player with that name already exist".to_string()).send(&mut server, user);
            return;
        }

        commands.spawn((Player, AdmiralBundle::new(&data.name, &data.race, "free traders")));

        ServerMessage::PlayerJoined(data.name.clone()).broadcast_except(&mut server, user);                
        ServerMessage::ConnectionSuccess.send(&mut server, user);
    }
}
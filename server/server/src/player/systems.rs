use bevy::prelude::*;
use bevy_websocket_server::{Server, ParsedMessages, generate_finder, SendableMessage, ReceivableMessage};

use crate::{readable::components::Name, admiral::{AdmiralBundle, Admiral}, map::{MapEntityMoveTarget, EntitySpawned, MapEntityId}};
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
    available_players: Query<(&Player, Entity, &Name), Without<ClientID>>,
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
                debug!("Login player {:?}", player_name);
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
/// If the name is unique, it creates a new player entity, sends EntitySpawned event and broadcasts a PlayerJoined message to all clients except the new one. 
/// Finally, it sends a ConnectionSuccess message to the new client to confirm successful registration.
pub fn register(
    mut server: ResMut<Server>, 
    mut commands: Commands,
    messages: Res<ParsedMessages<ClientMessage>>,
    mut spawn_ev: EventWriter<EntitySpawned>, 
    players: Query<(&Player, &Name)>
) {
    if let Some((user, data)) = generate_finder!(ClientMessage::Register, messages) {
        if players.into_iter().find(|(_, Name(name))| name == &data.name).is_some() {
            ServerMessage::RegistrationFailed("Player with that name already exist".to_string()).send(&mut server, user);
            return;
        }

        let admiral = AdmiralBundle::new(&data.name, &data.race, "free traders");
        let id = admiral.id();
        let entity: bevy::ecs::system::EntityCommands = commands.spawn((Player, admiral, ClientID(*user)));

        spawn_ev.send(EntitySpawned(MapEntityId::new(entity.id(), &id), Vec2::ZERO));

        debug!("Register player name: {:?}, race: {:?}", &data.name, &data.race);

        ServerMessage::PlayerJoined(data.name.clone()).broadcast_except(&mut server, user);                
        ServerMessage::ConnectionSuccess.send(&mut server, user);
    }
}

/// This system handles the MoveTo request.
/// 
/// It adds a new instance of the `MapEntityMoveTarget` component to the entity in order to force the `Map` plugin 
/// to update the player's position and rotation until it reaches the requested position.
pub fn request_move_player(
    mut commands: Commands,
    messages: Res<ParsedMessages<ClientMessage>>,
    players: Query<(Entity, &ClientID)>
) {
    if let Some((user, data)) = generate_finder!(ClientMessage::MoveTo, messages) {
        if let Some((entity, _)) = players.iter().find(|(_, ClientID(id))| *id == *user) {
            commands.entity(entity).insert(MapEntityMoveTarget(*data, 2.0));
        }
    }
}

/// This system handles the GetPlayerAdmiralId request.
pub fn request_admiral_id(
    mut server: ResMut<Server>,
    messages: Res<ParsedMessages<ClientMessage>>,
    players: Query<(Entity, &ClientID, &Admiral)>
) {
    if let Some((user, _)) = ClientMessage::GetPlayerAdmiralId().find(messages) {
        if let Some((_, _, Admiral(id))) = players.iter().find(|(_, ClientID(id), _)| *id == user) {
            ServerMessage::PlayerAdmiralId(*id).send(&mut server, &user);
        }
    }
}
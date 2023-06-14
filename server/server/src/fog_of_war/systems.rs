use super::{ messages::{AdmiralPacket, ServerMessage}, AdmiralsInSight, ViewRadius, FogOfWarUpdate, ClientMessage };
use crate::{ admiral::Admiral, fraction::{Fraction, Race}, map::{Map, MapEntity, MapEntityId}, readable::components::Name, player::ClientID };
use bevy::{prelude::*, utils::HashMap};
use bevy_websocket_server::{Server, SendableMessage, ParsedMessages};
use std::time::{SystemTime, UNIX_EPOCH};
use bevy_websocket_server::ReceivableMessage;

/// Minimal time (in ms) that should pass between two sight calculations.
const DELTA_UPDATE: u128 = 100;

/// This system calculates and updates the sight (`AdmiralsInSight` component) of all admirals.
///
/// If the calculated admiral is a player, it:
/// - Sends the `AdmiralAppears` message if a new player appears in sight.
/// - Sends the `AdmiralDisappears` message if a player disappears from sight.
/// - Sends the `PositionOfAdmiralsInSign` message with the position of all admirals in sight.
///
/// **Note:** The calculation will be done once per 100ms (can be configured by the `DELTA_UPDATE` const).
/// 
pub fn update_admirals_in_sign(
    mut server: ResMut<Server>,
    mut admirals: Query<( Entity, Option<&ClientID>, &Name, &Race, &Fraction, &mut AdmiralsInSight, &ViewRadius, &mut MapEntity, &Admiral)>,
    map: Res<Map<MapEntityId>>,
    mut update: ResMut<FogOfWarUpdate>,
) {
    let mut before = HashMap::<Entity, Vec<(MapEntityId, Vec2)>>::new();
    let time = get_utc_time();

    // Check if calculation is needed
    if time - update.last_update < DELTA_UPDATE {
        return;
    }

    update.last_update = time.clone();

    // Update the AdmiralsInSight components of admirals
    for (entity, _, _, _, _, mut sight, ViewRadius(view_radius), _, Admiral(admiral_id)) in admirals.iter_mut() {
        let map_id = MapEntityId::new(entity, &admiral_id);
        let actual = map.nearby(&map_id, *view_radius);
        before.insert(entity, sight.0.clone());
        sight.0 = actual.clone();

    }

    // Check for appearing and disappearing admirals
    for (entity, client_id, _, _, _, sight, _, _, _) in admirals.iter() {
        let actual = sight.0.clone();

        // All admirals that were sight in the previous sight calculation
        let removed = match before.get_mut(&entity) {
            Some(entity) => entity,
            None => continue,
        };

        // Skip current admiral if it is a npc
        let client_id = match client_id {
            Some(id) => id.0,
            None => continue,
        };

        for (entity, _) in actual.iter() {
            // Is entity in the removed vector?
            let mut is_old = false;
            
            // Remove entity from the removed vector to keep in it only disappeared admirals
            for (index_b, (entity_b, _)) in removed.iter().enumerate() {
                if entity_b == entity {
                    removed.remove(index_b);
                    is_old = true;
                    break;
                }
            }

            // Send AdmiralAppears messages to client
            if !is_old {
                if let Ok(admiral) = admirals.get(entity.entity) {
                    let packet = AdmiralPacket {
                        id: admiral.8.0.clone(),
                        name: admiral.2.0.clone(),
                        fraction: admiral.4.0.clone(),
                        race: admiral.3.0.clone(),
                        pos: admiral.7.0.clone(),
                        rotation: admiral.7.1.clone(),
                    };

                    ServerMessage::AdmiralAppears(packet).send(&mut server, &client_id);
                }
            }
        }

        // Send AdmiralDisappears messages to client
        for (id, _) in removed {
            ServerMessage::AdmiralDisappears(id.admiral).send(&mut server, &client_id);
        }

        let mut packet = (vec![], time);

        // Prepare the AdmiralPacket
        for (id, _) in sight.0.iter() {
            if let Ok(admiral) = admirals.get(id.entity) {
                packet.0.push((id.admiral, admiral.7 .0, admiral.7 .1));
            }
        }

        // Send the PositionOfAdmiralsInSign message
        ServerMessage::PositionOfAdmiralsInSign(packet).send(&mut server, &client_id);
    }
}

/// Returns the current UTC time in milliseconds since the Unix epoch.
///
/// # Panics
///
/// This function will panic if it fails to retrieve the time since the Unix epoch.
///
fn get_utc_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Cannot get time sine unix epoch!")
        .as_millis()
}

/// This system handles the client's GetAllAdmiralsInSign requests.
pub fn get_admirals_in_sign(
    mut server: ResMut<Server>,
    admirals: Query<( Entity, Option<&ClientID>, &Name, &Race, &Fraction, &AdmiralsInSight, &ViewRadius, &MapEntity, &Admiral)>,
    messages: Res<ParsedMessages<ClientMessage>>,
) {
    if let Some((user, _)) = ClientMessage::GetAllAdmiralsInSign().find(messages) {
        if let Some(player) = admirals.iter().find(|a| a.1.is_some_and(|a| a.0 == user)) {
            let ClientID(client_id) = player.1.expect("Logged player instance must have an client component!");

            for (entity, _) in player.5.0.iter() {
                if let Ok(admiral) = admirals.get(entity.entity) {
                    let packet = AdmiralPacket {
                        id: admiral.8.0.clone(),
                        name: admiral.2.0.clone(),
                        fraction: admiral.4.0.clone(),
                        race: admiral.3.0.clone(),
                        pos: admiral.7.0.clone(),
                        rotation: admiral.7.1.clone(),
                    };

                    ServerMessage::AdmiralAppears(packet).send(&mut server, client_id);
                }
            }
        }
    }
}
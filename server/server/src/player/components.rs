use bevy::prelude::*;

/// Component that represents the unique identifier for a connected client.
/// All connected players should have this component, and it should be removed if the player disconnects.
#[derive(Component)]
pub struct ClientID(pub u64);

/// Component that marks an entity as a player in the game.
///
/// Note that even disconnected players have this component. When a player is disconnected, their entity is not deleted, but remains in the game
/// to inform other players that the player's name is already in use.
///
/// If you want to query only connected players, use the `ClientID` component instead.
#[derive(Component)]
pub struct Player;
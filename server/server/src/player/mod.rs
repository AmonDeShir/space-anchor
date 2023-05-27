mod components;
mod systems;
mod messages;

use bevy_websocket_server::{ParsedMessages, NetworkSystemSet, ReceivableMessage};
pub use components::*;

use bevy::prelude::*;

use self::{systems::{login, register}, messages::ClientMessage};

/// Plugin that adds system for player creation and connection.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ParsedMessages<ClientMessage>>()
            .add_system(ClientMessage::parse.in_set(NetworkSystemSet::EventParser))
            .add_system(register.in_set(NetworkSystemSet::EventReady))
            .add_system(login.in_set(NetworkSystemSet::EventReady));
    }
}
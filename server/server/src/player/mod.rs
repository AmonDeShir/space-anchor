mod components;
mod systems;
mod messages;

pub use components::*;

use bevy::prelude::*;
use ::messages::{ParsedMessages, ReceivableMessage};

use crate::networking::NetworkSystemSet;
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
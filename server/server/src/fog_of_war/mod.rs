mod components;
mod messages;
mod systems;
mod resources;

use bevy::prelude::{Plugin, App, IntoSystemConfig};
use bevy_websocket_server::{ParsedMessages, NetworkSystemSet, ReceivableMessage};
pub use components::*;
pub use messages::*;
pub use resources::*;
use systems::*;

/// Plugin that adds system for calculating admirals sight
pub struct FogOfWarPlugin;

impl Plugin for FogOfWarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ParsedMessages<ClientMessage>>()
            .insert_resource(FogOfWarUpdate { last_update: 0 })
            .add_system(ClientMessage::parse.in_set(NetworkSystemSet::EventParser))
            .add_system(get_admirals_in_sign.in_set(NetworkSystemSet::EventReady))
            .add_system(update_admirals_in_sign);
    }
}
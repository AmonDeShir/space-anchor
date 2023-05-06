mod resources;
mod systems;
mod messages;
pub mod version;

use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use ::messages::{ParsedMessages, ReceivableMessage, UnparsedMessages};

use self::systems::*;
use self::messages::*;
use resources::*;

/// This enum defines system sets that handle the different stages of the network message lifetime.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum NetworkSystemSet {
    /// This system set includes the systems responsible for parsing raw Renet messages and saving them as strings in the UnparsedMessages resource. 
    /// All systems that parse these strings and convert them into usable messages should belong to this set.
    EventParser,
    /// This system set includes the systems that parse the parsed strings into corresponding resources that implement the ReceivableMessage trait.
    EventReady,
}

/// This plugin is responsible for setting up the networking systems in the game.
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
         app.configure_set(NetworkSystemSet::EventParser.before(NetworkSystemSet::EventReady))
            .add_plugin(RenetServerPlugin::default())
            .init_resource::<UnparsedMessages>()
            .init_resource::<ParsedMessages<ClientMessage>>()
            .insert_resource(new_renet_server())
            .add_system(handle_renet_server_events)
            .add_system(handle_received_messages.before(NetworkSystemSet::EventParser))
            .add_system(ClientMessage::parse.in_set(NetworkSystemSet::EventParser))
            .add_system(handle_ping_request.in_set(NetworkSystemSet::EventReady));
    }
}
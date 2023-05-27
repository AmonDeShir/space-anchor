mod systems;
mod messages;

pub mod version;

use bevy::prelude::*;
use bevy_websocket_server::WebsocketServer;

use self::systems::*;
use self::messages::*;

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
            .add_plugin(WebsocketServer)
            .add_system(handle_server_events);
    }
}
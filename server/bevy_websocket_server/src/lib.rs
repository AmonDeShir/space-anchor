mod message;
mod server;
mod systems;
mod events;

pub use message::*;
pub use server::*;
pub use events::*;
use systems::*;

use bevy::prelude::*;

/// This enum defines system sets that handle the different stages of the network message lifetime.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum NetworkSystemSet {
    /// This system set includes the systems responsible for parsing messages and saving them as strings in the UnparsedMessages resource. 
    /// All systems that parse these strings and convert them into usable messages should belong to this set.
    EventParser,
    /// This system set includes the systems that parse the parsed strings into corresponding resources that implement the ReceivableMessage trait.
    EventReady,
}

/// This plugin is responsible for setting up the networking system
pub struct WebsocketServer;

impl Plugin for WebsocketServer {
    fn build(&self, app: &mut App) {
         app.configure_set(NetworkSystemSet::EventParser.before(NetworkSystemSet::EventReady))
            .init_resource::<UnparsedMessages>()
            .insert_resource(Server::new())
            .add_event::<ClientConnected>()
            .add_event::<ClientDisconnected>()
            .add_startup_system(startup)
            .add_system(handle_accept_queue)
            .add_system(handle_server_events.before(NetworkSystemSet::EventParser));
    }
}
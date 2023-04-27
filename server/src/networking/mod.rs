mod resources;
mod systems;
mod messages;
pub mod version;
pub mod channel;

pub use resources::*;
pub use messages::*;

use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use resources::new_renet_server;

use self::systems::*;


pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RenetServerPlugin::default())
            .init_resource::<CurrentServerMessages>()
            .insert_resource(new_renet_server())
            .add_system(handle_server_connection)
            .add_system(server_recieve_messages)
            .add_system(server_ping_pong);
    }
}
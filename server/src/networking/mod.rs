pub mod resources;
mod systems;

use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;
use systems::new_renet_server;


pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RenetServerPlugin::default())
            .insert_resource(new_renet_server());
    }
}
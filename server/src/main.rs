pub mod networking;
pub mod readable;
pub mod fraction;
pub mod admiral;
pub mod player;

use bevy::prelude::*;
use networking::NetworkingPlugin;

fn main() {
    App::new()
        .add_plugin(AssetPlugin::default())        
        .add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin)
        .run();
}
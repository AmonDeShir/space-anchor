pub mod networking;
pub mod readable;
pub mod fraction;
pub mod admiral;
pub mod player;

use std::time::Duration;

use bevy::{prelude::*, log::LogPlugin, app::ScheduleRunnerSettings};
use networking::NetworkingPlugin;
use player::PlayerPlugin;


fn main() {
    App::new()
        .add_plugin(LogPlugin {
            filter: "debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(1.0 / 60.0)))
        .add_plugin(AssetPlugin::default())        
        .add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
use bevy::prelude::Resource;

/// Time (in milliseconds) since the last admiral's sight calculation.
#[derive(Resource)]
pub struct FogOfWarUpdate {
    pub last_update: u128,
}
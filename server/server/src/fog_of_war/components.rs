use bevy::prelude::{Component, Vec2};
use crate::map::MapEntityId;

/// The radius of admiral's visibility zone.
#[derive(Component)]
pub struct ViewRadius(pub f32);

/// All admirals that are visible by the player or admiral.
///
/// **Note** that it contains all admirals within the radius of the player, including the player itself.
/// 
/// ## Arguments
/// 
/// Vector of tuples:
/// * `MapEntityId` - Id of an admiral
/// * `Vec2` - Admiral position
#[derive(Component)]
pub struct AdmiralsInSight(pub Vec<(MapEntityId, Vec2)>);

impl Default for ViewRadius {
    fn default() -> Self {
        ViewRadius(10.0)
    }
}
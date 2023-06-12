use bevy::prelude::{Component, Vec2};

use super::CellId;

#[derive(Component)]
/// Basic map entity that contains information about its world position and rotation, as well as the map cell
/// in which the entity is located.
/// 
/// ## Arguments
/// 
/// * `Vec2` - Position on the map.
/// * `f32` - Entity rotation in the x-axis in radians.
/// * `CellId` - ID of the cell in which the entity is located.
pub struct MapEntity(pub Vec2, pub f32, pub CellId);

#[derive(Component)]
/// Component for moving entities. If this component is added to an entity that also has a MapEntity component.
/// The map system will move the entity until it reaches the target position. After that, this component will
/// be automatically removed from the entity.
/// 
/// ## Arguments
/// 
/// * `Vec2` - Target location.
/// * `f32` - Maximum speed of the entity.
pub struct MapEntityMoveTarget(pub Vec2, pub f32);
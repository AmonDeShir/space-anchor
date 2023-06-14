use bevy::prelude::Vec2;

use super::{CellId, MapEntityId};

/// Event emitted when an entity is spawned in the world, along with its initial position.
/// 
/// ## Arguments
/// 
/// * `MapEntityId` - ID of the map entity.
/// * `Vec2` - Position of the entity.
/// 
pub struct EntitySpawned(pub MapEntityId, pub Vec2);

/// Event emitted when an entity is removed from the world.
/// 
/// ## Arguments
///
/// * `MapEntityId` - ID of the map entity.
///
pub struct EntityRemoved(pub MapEntityId);

/// Event emitted when an entity changes its cell in the map, along with its new position and cell ID.
/// 
/// ## Arguments
/// 
/// * `MapEntityId` - ID of the map entity.
/// * `Vec2` - Updated position of the entity.
/// * `CellId` - I of the new entity's cell.
/// 
pub struct EntityChangedCell(pub MapEntityId, pub Vec2, pub CellId);
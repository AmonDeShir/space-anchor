use bevy::prelude::{Entity, Vec2};

use super::CellId;

/// Event emitted when an entity is spawned in the world, along with its initial position.
pub struct EntitySpawned(pub Entity, pub Vec2);

/// Event emitted when an entity is removed from the world.
pub struct EntityRemoved(pub Entity);

/// Event emitted when an entity changes its cell in the map, along with its new position and cell ID.
pub struct EntityChangedCell(pub Entity, pub Vec2, pub CellId);

/// Event emitted when an entity's position is updated in the world.
pub struct EntityMoved(pub Entity, pub Vec2);

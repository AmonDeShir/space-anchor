use bevy::prelude::*;

mod events;
mod resources;
mod systems;
mod components;

pub use components::*;
pub use resources::*;
pub use events::*;

use self::systems::*;

/// Id of an entity on the map
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct MapEntityId {
    pub entity: Entity,
    pub admiral: u64,
}

impl MapEntityId {
    /// Creates a new MapEntityId with the given entity and admiral ID.
    pub fn new(entity: Entity, id: &u64) -> MapEntityId {
        MapEntityId { entity, admiral: *id }
    }
}

/// Plugin for managing the game map and entity positions.
///
/// The MapPlugin sets up the necessary resources and events for the map system to function.
/// It provides the Map resource, which stores the entity positions and cells.
/// It also adds the required events, such as EntitySpawned, EntityRemoved, EntityChangedCell, and EntityMoved.
/// 
/// The plugin registers the spawn_or_despawn_entity system, which handles entity spawning and removal,
/// and the update_entity_position system, which updates entity positions and triggers cell change events.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Map::<MapEntityId>::new(100.0))
            .add_event::<EntitySpawned>()
            .add_event::<EntityRemoved>()
            .add_event::<EntityChangedCell>()
            .add_system(spawn_or_despawn_entity)
            .add_system(update_entity_position);
    }
}
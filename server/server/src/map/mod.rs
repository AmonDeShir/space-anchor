use bevy::prelude::*;

mod events;
mod resources;
mod systems;
pub use resources::*;
pub use events::*;

use self::systems::*;

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
            .insert_resource(Map::new(100))
            .add_event::<EntitySpawned>()
            .add_event::<EntityRemoved>()
            .add_event::<EntityChangedCell>()
            .add_event::<EntityMoved>()
            .add_system(spawn_or_despawn_entity)
            .add_system(update_entity_position);
    }
}
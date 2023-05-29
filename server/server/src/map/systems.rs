use bevy::prelude::*;

use super::{EntitySpawned, Map, EntityRemoved, EntityChangedCell, EntityMoved};

/// Adds or removes entities from the map based on spawned and removed events, and triggers `EntityChangedCell` event when an entity is added to the map.
pub fn spawn_or_despawn_entity(
  mut spawned_ev: EventReader<EntitySpawned>,
  mut removed_ev: EventReader<EntityRemoved>,
  mut changed_cell_ev: EventWriter<EntityChangedCell>,
  mut map: ResMut<Map>,
) {  
  for EntityRemoved(entity) in removed_ev.iter() {
    map.remove(entity);
  }

  for EntitySpawned(entity, pos) in spawned_ev.iter() {
    let cell_id = map.add(entity, *pos);

    changed_cell_ev.send(EntityChangedCell(*entity, *pos, cell_id));
  }
}


/// Updates the positions of entities based on the `EntityMoved` event, and triggers `EntityChangedCell`
/// events when an entity changes its cell.
pub fn update_entity_position(
  mut moved_ev: EventReader<EntityMoved>,
  mut changed_cell_ev: EventWriter<EntityChangedCell>,
  mut map: ResMut<Map>,
) {
  for EntityMoved(entity, pos) in moved_ev.iter() {
    if let Some(cell_id) = map.update(entity, *pos) {
      changed_cell_ev.send(EntityChangedCell(*entity, *pos, cell_id));
    }
  }
}
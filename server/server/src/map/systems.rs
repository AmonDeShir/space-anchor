use bevy::prelude::*;
use crate::admiral::Admiral;
use std::f32::consts::PI;
use super::{EntitySpawned, Map, EntityRemoved, EntityChangedCell, MapEntity, MapEntityMoveTarget, MapEntityId};

/// Adds or removes entities from the map based on spawned and removed events, and triggers `EntityChangedCell` event when an entity is added to the map.
pub fn spawn_or_despawn_entity(
    mut commands: Commands,
    mut spawned_ev: EventReader<EntitySpawned>,
    mut removed_ev: EventReader<EntityRemoved>,
    mut changed_cell_ev: EventWriter<EntityChangedCell>,
    mut map: ResMut<Map<MapEntityId>>,
) {  
    for EntityRemoved(entity) in removed_ev.iter() {
        map.remove(entity);
        commands.entity(entity.entity).remove::<(MapEntity, MapEntityMoveTarget)>();
    }

    for EntitySpawned(entity, pos) in spawned_ev.iter() {
        let cell_id = map.add(entity, *pos);

        changed_cell_ev.send(EntityChangedCell(*entity, *pos, cell_id));
        commands.entity(entity.entity).insert(MapEntity(*pos, 0.0, cell_id));
    }
}

/// Updates the position of entities based on their target position and speed.
///
/// Triggers EntityChangedCell event when an entity changes its cell on the map.
pub fn update_entity_position(
    mut commands: Commands,
    mut entities: Query<(Entity, &Admiral, &mut MapEntity, &MapEntityMoveTarget)>,
    mut changed_cell_ev: EventWriter<EntityChangedCell>,
    mut map: ResMut<Map<MapEntityId>>,
    time: Res<Time>,
) {
    for (entity, Admiral(admiral), mut map_entity, MapEntityMoveTarget(target, speed)) in entities.iter_mut() {
        let MapEntity(pos, angle, cell_id) = map_entity.as_mut();

        let direction = *target - *pos;
        let distance = speed * time.delta_seconds();

        // Check if the object is already close enough to the target to stop the movement
        if distance >= direction.length() {
            *pos = *target;

            // Remove the MapEntityMoveTarget Component as entity reach the target
            commands.entity(entity).remove::<MapEntityMoveTarget>();
        } 
        else {
            // Move
            let displacement = direction.normalize() * distance;
            *pos += displacement;
        }

        // Calculate the rotation
        // We use an inverted cartesian system, so for all mathematical operations x is y and y is x.
        let facing_vec = *pos - *target;
        let target_angle = f32::atan2(facing_vec.x, facing_vec.y);

        // Interpolate rotation for smooth animation
        let lerp_factor = 2.0;
        *angle = lerp_angle(*angle, target_angle, lerp_factor * time.delta_seconds());

        let map_id = MapEntityId::new(entity, admiral);

        // Update the object's position on the map
        if let Some(new_cell_id) = map.update(&map_id, *pos) {
            changed_cell_ev.send(EntityChangedCell(map_id, *pos, new_cell_id));
            *cell_id = new_cell_id;
        }
    }
}
 
/// Performs linear interpolation (lerp) between two angles.
///
/// ## Arguments
/// 
/// * `start` - The starting value.
/// * `end` - The ending value.
/// * `t` - The interpolation factor in the range [0, 1].
/// 
fn lerp_angle(start: f32, end: f32, t: f32) -> f32 {
  start + short_angle_dist(start, end) * t
}

/// Calculates the shortest angle distance between two angles.
///
/// # Arguments
///
/// * `from` - The starting angle in radians.
/// * `to` - The target angle in radians.
///
///
fn short_angle_dist(from: f32, to: f32) -> f32 {
  let max_angle: f32 = PI * 2.0;
  let difference = (to - from).rem_euclid(max_angle);
  return ((2.0 * difference).rem_euclid(max_angle)) - difference;
}
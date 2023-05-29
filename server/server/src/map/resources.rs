use bevy::{prelude::{Resource, Vec2, Entity}, utils::HashMap};

pub const TILE_SIZE: usize = 1;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct CellId {
  pub x: i32,
  pub y: i32,
}

impl CellId {
  pub fn new(x: i32, y: i32) -> CellId {
    CellId {
      x,
      y
    }
  }
}


#[derive(Resource)]
pub struct Map {
  dimension_size: f32,
  cells: HashMap<CellId, HashMap<Entity, Vec2>>,
  entities: HashMap<Entity, CellId>
}

impl Map {
  pub fn new(dimension_size: f32) -> Map {
    Map {
      dimension_size,
      cells: HashMap::new(),
      entities: HashMap::new()
    }
  }

  /// Places an entity in the world at the specified position and returns the ID of the cell where the entity is placed.
  ///
  /// * `id` - The reference to the entity to be placed.
  /// * `pos` - The position vector where the entity should be placed (`Vec2`).
  ///
  pub fn add(&mut self, id: &Entity, pos: Vec2) -> CellId {
    let cell_id = self.pos_to_cell(&pos);
    let cell = self.cells.entry(cell_id).or_insert_with(HashMap::new);

    cell.insert(*id, pos);

    return cell_id;
  }

  /// Converts a world position to a corresponding cell ID.
  fn pos_to_cell(&self, pos: &Vec2) -> CellId {
    let x = (pos.x / self.dimension_size).floor() as i32;
    let y = (pos.y / self.dimension_size).floor() as i32;

    return CellId::new(x, y);
  }

  /// Removes an entity from the map. If the entity is the last item in its cell, the cell is also removed.
  ///
  /// It does nothing if the entity is not present in the map.
  ///
  /// * `id` - The reference to the entity to be removed.
  pub fn remove(&mut self, id: &Entity) {
    let cell_id = match self.entities.get(id) {
      Some(cell) => cell,
      None => return,
    };

    if let Some(cell) = self.cells.get_mut(cell_id) {
      if cell.len() == 1 {
        self.cells.remove(&cell_id);
      }
      else {
        cell.remove(id);
      }
    }

    self.entities.remove(id);
  }

  /// Updates the position of an entity and, if necessary, moves it to the next cell.  
  ///
  /// Returns `Some(CellId)` if the entity changed cells, indicating the new cell ID.  
  /// Returns `None` if the entity is not present in the map or if the cell did not change.
  ///
  /// * `id` - The reference to the entity to be updated.
  /// * `pos` - The new position vector for the entity (`Vec2`).
  pub fn update(&mut self, id: &Entity, pos: Vec2) -> Option<CellId> {
    let new_cell_id = self.pos_to_cell(&pos);

    if let Some(entity_cell) = self.entities.get_mut(id) {
      let old_cell_id = *entity_cell;
      let cell = self.cells.get_mut(&old_cell_id).expect("Map's cell does not exist!");

      if old_cell_id != new_cell_id {
        cell.remove(id);

        let new_cell = self.cells.entry(new_cell_id).or_insert_with(HashMap::new);
        new_cell.insert(*id, pos);

        *entity_cell = new_cell_id;
        return Some(new_cell_id);
      }

      // Update entity positon
      *cell.get_mut(id).expect("Map's cell does not contains searched entity!") = pos;
    }
    
    return None;
  }

  /// Returns a list of entities within a specified radius of the given `id` element.
  ///
  /// * `id` - The reference to the entity for which to find nearby entities.
  /// * `radius` - The radius within which to search for nearby entities.
  ///
  /// ## Returns
  ///
  /// A vector containing tuples of nearby entities and their corresponding positions (`Vec<(Entity, Vec2)>`).
  ///
  pub fn nearby(&self, id: &Entity, radius: f32) -> Vec<(Entity, Vec2)> {
    let mut entities = vec![];

    if let Some(cell_id) = self.entities.get(&id) {
      let center = self.force_get(cell_id, id);
      let cells = self.get_cells(&self.get_id_of_nearby_cells(cell_id, &radius));

      for (entity, pos) in cells {
        if Map::is_in_circle(&center, &radius, &pos) {
          entities.push((entity, pos));
        }
      }
    }

    return entities;
  }


  /// Retrieves the position of an entity from the specified cell.
  ///
  /// ## Arguments
  ///
  /// * `cell_id` - The reference to the cell ID where the entity is located.
  /// * `id` - The reference to the entity for which to retrieve the position.
  ///
  /// ## Returns
  ///
  /// The position of the entity in the specified cell (`Vec2`).
  ///
  /// ## Panics
  ///
  /// This function will panic if the specified cell does not exist or if it does not contain the searched entity.
  ///
  fn force_get(&self, cell_id: &CellId, id: &Entity) -> Vec2 {
    *self.cells.get(cell_id).expect("Map's cell does not exist!").get(id).expect("Map's cell does not contains searched entity!") 
  }


  /// Retrieves the entities and their positions from the specified cells.
  fn get_cells(&self, cells: &Vec<CellId>) -> Vec<(Entity, Vec2)> {
    let mut result = vec![];
    
    for cell in cells.iter() {
      if let Some(map) = self.cells.get(cell) {
        for (key, pos) in map.iter() {
          result.push((*key, *pos));
        }
      }
    }

    return result;
  }

  /// Retrieves the IDs of the nearby cells within the specified radius of the given cell ID.
  ///
  /// * `id` - The reference to the cell ID for which to retrieve nearby cell IDs.
  /// * `radius` - The reference to the radius value (`f32`).
  ///
  fn get_id_of_nearby_cells(&self, id: &CellId, radius: &f32) -> Vec<CellId> {
    let radius = (radius / self.dimension_size).round() as i32;

    let min_x = id.x - radius;
    let min_y = id.y - radius;
    let max_x = id.x + radius;
    let max_y = id.y + radius;

    let mut ids = vec![];

    for x in min_x..=max_x {
      for y in min_y..=max_y {
        ids.push(CellId::new(x, y));
      }
    }

    return ids;
  }

  /// Checks if a given position is within the specified circle defined by the center position and radius.
  ///
  /// * `center` - The reference to the center position vector (`Vec2`).
  /// * `radius` - The reference to the radius value (`f32`).
  /// * `pos` - The reference to the position vector to be checked (`Vec2`).
  fn is_in_circle(center: &Vec2, radius: &f32, pos: &Vec2) -> bool {
    let pos = (*center - *pos).abs();

    return pos.x <= *radius && pos.y <= *radius; 
  }
}


use bevy::prelude::*;

// Name of an entity (ship, item, npc, planet, etc) 
#[derive(Component)]
pub struct Name(pub String);

// Description of an entity (ship, item, npc, planet, etc) 
#[derive(Component)]
pub struct Description(pub String);


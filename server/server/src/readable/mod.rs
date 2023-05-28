pub mod components;

pub use components::*;

use components::Name;
use bevy::prelude::*;

// Bundle that provides name and description for all kind of game entities - from single ship to star systems. 
#[derive(Bundle)]
pub struct ReadableBundle {
    name: Name,
    description: Description
}

impl ReadableBundle {
    /// Creates a bundle with Name and Description components
    pub fn new(name: &str, description: &str) -> ReadableBundle {
        ReadableBundle {
            name: Name(name.to_string()), 
            description: Description(description.to_string())
        }
    }
}
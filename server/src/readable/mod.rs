pub mod components;

pub use components::*;

use components::Name;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ReadableBundle {
    name: Name,
    description: Description
}

impl ReadableBundle {
    fn new(name: &str, description: &str) -> ReadableBundle {
        ReadableBundle {
            name: Name(name.to_string()), 
            description: Description(description.to_string())
        }
    }
}
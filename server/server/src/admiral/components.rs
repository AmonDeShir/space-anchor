use std::sync::atomic::{AtomicU64, Ordering};

use bevy::prelude::*;

/// Components for all entities that can move over the map with a fleet.
///
/// ## Arguments
///
/// * `u64` - Id of an admiral, should be unique and shouldn't be changed.
/// 
/// **Note:** Use Admiral::new to create a new component. It will ensure that the id is unique.
#[derive(Component)]
pub struct Admiral(pub u64);

impl Admiral {
    /// Creates a new instance of the Admiral component with a unique id.
    pub fn new() -> Admiral {
        Admiral(Admiral::next_id())
    }

    /// Generates a unique admiral ID.
    fn next_id() -> u64 {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}

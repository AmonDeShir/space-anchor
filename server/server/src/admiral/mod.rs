mod components;

pub use components::*;
use bevy::prelude::*;

use crate::readable::components::Name;
use crate::fraction::{Race, Fraction};

use self::components::Admiral;

#[derive(Bundle)]
pub struct AdmiralBundle {
    admiral: Admiral,
    name: Name,
    race: Race,
    fraction: Fraction
}

impl AdmiralBundle {
    pub fn new(name: &str, race: &str, fraction: &str) -> AdmiralBundle {
        AdmiralBundle {
            admiral: Admiral,
            name: Name(name.to_string()),
            race: Race(race.to_string()),
            fraction: Fraction(fraction.to_string()),
        }
    }
}
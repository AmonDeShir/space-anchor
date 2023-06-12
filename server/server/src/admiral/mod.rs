mod components;

pub use components::*;
use bevy::prelude::*;

use crate::fog_of_war::{ViewRadius, AdmiralsInSight};
use crate::readable::components::Name;
use crate::fraction::{Race, Fraction};

/// Bundle that contains all components needed to create a movable entity.
///
/// ## Components
///
/// * `Admiral` - Id of an admiral
/// * `Name` - Name of the entity
/// * `Race` - Race of the entity
/// * `Fraction` - Fraction of the entity
/// * `ViewRadius` - The radius of the admiral's visibility zone.
/// * `AdmiralsInSight` - All admirals that are in the admiral's visibility range.
/// 
#[derive(Bundle)]
pub struct AdmiralBundle {
    admiral: Admiral,
    name: Name,
    race: Race,
    fraction: Fraction,
    view_radius: ViewRadius,
    sight: AdmiralsInSight,
}

impl AdmiralBundle {
    /// Creates new admiral
    pub fn new(name: &str, race: &str, fraction: &str) -> AdmiralBundle {
        AdmiralBundle {
            admiral: Admiral::new(),
            name: Name(name.to_string()),
            race: Race(race.to_string()),
            fraction: Fraction(fraction.to_string()),
            view_radius: ViewRadius::default(),
            sight: AdmiralsInSight(vec![]),
        }
    }

    /// Return the admiral's id
    pub fn id(&self) -> u64 {
        self.admiral.0
    }
}
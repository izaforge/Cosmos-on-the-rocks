use bevy::prelude::*;
use std::collections::HashMap;

use crate::bar::{glass::Glass, ingredient::{PrimaryEffect, SecondaryEffect}};

#[derive(Component, Debug)]
pub struct Drink {
    pub name: String,
    pub ingredients: HashMap<Entity, f32>,
    pub glass: Entity,
}

impl From<(Glass, Entity)> for Drink {
    fn from((glass, glass_entity): (Glass, Entity)) -> Self {
        Drink {
            name: format!("{:#?}", glass.shape),
            ingredients: glass.ingredients,
            glass: glass_entity,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum HazardEffect {
    VisualAuditoryGlitches,
    SuddenConfession,
    PersonalityFlip,
    CosmicDistortion,
    MemoryLeak,
    CloneEffect,
}

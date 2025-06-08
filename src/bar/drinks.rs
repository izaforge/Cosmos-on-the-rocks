use bevy::prelude::*;
use std::collections::HashMap;

use crate::bar::{
    glass::{Glass, GlassShape},
    ingredient::{IngredientTaste, PrimaryEffect, SecondaryEffect},
};

#[derive(Component, Debug)]
pub struct Drink {
    pub name: String,
    pub ingredients: HashMap<Entity, f32>,
    pub taste: DrinkTaste,
    pub created_drink: CreatedDrink,
}

#[derive(Debug)]
pub enum CreatedDrink {
    ZeroPhase,
    CryoDrop,
    Cosmopolitan,
    SynthCascade,
    OldMemory,
    EchoBloom,
    BotanicalSurge,
    BinaryBarrel,
    EventHorizon,
}

impl From<Glass> for Drink {
    fn from(glass: Glass) -> Self {
        let mut taste_vec: Vec<(IngredientTaste, f32)> = glass.taste.into_iter().collect();
        taste_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let primary = taste_vec
            .get(0)
            .map(|(t, _)| *t)
            .unwrap_or(IngredientTaste::None);
        let secondary = taste_vec
            .get(1)
            .map(|(t, _)| *t)
            .unwrap_or(IngredientTaste::None);

        let tastes = DrinkTaste {
            primary_taste: primary,
            secondary_taste: secondary,
        };

        let created_drink = match glass.shape {
            GlassShape::Wine => match (primary, secondary) {
                (IngredientTaste::Sour, _) => CreatedDrink::BinaryBarrel,
                (IngredientTaste::Umami, _) => CreatedDrink::BotanicalSurge,
                (IngredientTaste::Sweet, IngredientTaste::Spicy) => CreatedDrink::EventHorizon,
                _ => CreatedDrink::ZeroPhase,
            },
            GlassShape::Whiskey => match (primary, secondary) {
                (IngredientTaste::Umami, _) => CreatedDrink::EchoBloom,
                (IngredientTaste::Bitter, _) => CreatedDrink::OldMemory,
                _ => CreatedDrink::CryoDrop,
            },
            GlassShape::Cocktail => match (primary, secondary) {
                (IngredientTaste::Citrus, _) => CreatedDrink::Cosmopolitan,
                (IngredientTaste::Spicy, _) => CreatedDrink::SynthCascade,
                _ => CreatedDrink::ZeroPhase,
            },
        };
        Drink {
            name: format!("{:#?}", created_drink),
            ingredients: glass.ingredients,
            taste: tastes,
            created_drink: created_drink,
        }
    }
}

#[derive(Debug)]
pub struct DrinkTaste {
    pub primary_taste: IngredientTaste,
    pub secondary_taste: IngredientTaste,
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

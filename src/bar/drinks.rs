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
    pub effect: DrinkEffect,
    pub created_drink: CreatedDrink,
}

#[derive(Debug)]
pub struct DrinkTaste {
    pub primary_taste: IngredientTaste,
    pub secondary_taste: IngredientTaste,
}

#[derive(Debug)]
pub struct DrinkEffect {
    pub primary_effect: PrimaryEffect,
    pub secondary_effect: PrimaryEffect,
}

#[derive(Debug)]
pub enum CreatedDrink {
    ZeroPhase,
    CryoDrop,
    StellarLumen,
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
        let primary_taste = taste_vec
            .get(0)
            .map(|(t, _)| *t)
            .unwrap_or(IngredientTaste::None);
        let secondary_taste = taste_vec
            .get(1)
            .map(|(t, _)| *t)
            .unwrap_or(IngredientTaste::None);

        let tastes = DrinkTaste {
            primary_taste: primary_taste,
            secondary_taste: secondary_taste,
        };

        let mut effects_vec: Vec<(PrimaryEffect, f32)> = glass.effect.into_iter().collect();
        effects_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let primary_effect = effects_vec
            .get(0)
            .map(|(t, _)| *t)
            .unwrap_or(PrimaryEffect::Calming);
        let secondary_effect = effects_vec
            .get(1)
            .map(|(t, _)| *t)
            .unwrap_or(PrimaryEffect::Calming);

        let effects = DrinkEffect {
            primary_effect: primary_effect,
            secondary_effect: secondary_effect,
        };

        let created_drink = match glass.shape {
            GlassShape::Wine => match (primary_taste, secondary_taste) {
                (IngredientTaste::Sour, _) => CreatedDrink::BinaryBarrel,
                (IngredientTaste::Umami, _) => CreatedDrink::BotanicalSurge,
                (IngredientTaste::Sweet, IngredientTaste::Spicy) => CreatedDrink::EventHorizon,
                _ => CreatedDrink::StellarLumen,
            },
            GlassShape::Whiskey => match (primary_taste, secondary_taste) {
                (IngredientTaste::Umami, _) => CreatedDrink::EchoBloom,
                (IngredientTaste::Bitter, _) => CreatedDrink::OldMemory,
                _ => CreatedDrink::CryoDrop,
            },
            GlassShape::Cocktail => match (primary_taste, secondary_taste) {
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
            effect: effects,
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

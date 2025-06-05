use bevy::prelude::*;
use uuid::Uuid;

use crate::animation::sprite_animation::SpriteAnimState;

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform, SpriteAnimState)]
pub struct Ingredient {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub ingredient_profile: IngredientProfile,
}

#[derive(Clone, Debug)]
pub struct IngredientProfile {
    pub size: f32,
    pub taste: IngredientTaste,
    pub primary_effect: PrimaryEffect,
    pub secondary_effect: SecondaryEffect,
    pub hazard: Option<String>,
}

#[derive(Clone, Debug)]
pub enum IngredientTaste {
    None,
    Sweet,
    Sour,
    Bitter,
    Salty,
    Umami,
    Spicy,
}

#[derive(Clone, Debug)]
pub enum PrimaryEffect {
    Calming,
    Energizing,
    MindEnhancing,
    CourageBoosting,
    TruthInducing,
    Healing,
}

#[derive(Clone, Debug)]
pub enum SecondaryEffect {
    Euphoric(EffectCondition),
    Agitated(EffectCondition),
    Hallucinogenic(EffectCondition),
    Paranoia(EffectCondition),
    Aggresive(EffectCondition),
    Sedated(EffectCondition),
}

#[derive(Clone, Debug)]
pub struct EffectCondition {
    pub volume_needed: f32,
    pub catalyst: Option<Uuid>,
}

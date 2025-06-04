use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Clone, Debug)]
pub struct Ingredient {
    pub id: uuid::Uuid,
    pub name: String,
    pub ingredient_profile: IngredientProfile
}

#[derive(Clone, Debug)]
pub struct IngredientProfile {
    pub taste: IngredientTaste,
    pub primary_effect: PrimaryEffect,
    pub secondary_effect: SecondaryEffect,
    pub hazard: String,
}

#[derive(Clone, Debug)]
pub enum IngredientTaste {
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
    Euphoric { conditon: EffectCondition },
    Agitated { conditon: EffectCondition },
    Hallucinogenic { conditon: EffectCondition },
    Paranoia { conditon: EffectCondition },
    Aggresive { conditon: EffectCondition },
    Sedated { conditon: EffectCondition },
}

#[derive(Clone, Debug)]
pub struct EffectCondition {
    volume_needed: f32,
    catalyst: Option<Uuid>,
}
use bevy::prelude::*;

use crate::bar::ingredient::{Ingredient, IngredientProfile, IngredientTaste, PrimaryEffect, SecondaryEffect, EffectCondition};

pub fn get_small_ice_gels(
    blue_icegel_sprite: Sprite,
    red_icegel_sprite: Sprite,
    green_icegel_sprite: Sprite,
) -> Vec<(Ingredient, Sprite, Transform)> {
    // Profiles for smaller ingredients
    let small_blue_icegel_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::None,
        primary_effect: PrimaryEffect::Calming,
        secondary_effect: SecondaryEffect::Sedated(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };
    let small_red_icegel_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::Spicy,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Agitated(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };
    let small_green_icegel_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::Sweet,
        primary_effect: PrimaryEffect::Healing,
        secondary_effect: SecondaryEffect::Euphoric(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    // Smaller ingredients
    let small_blue_icegel = Ingredient {
        name: "Small Blue Icegel".to_string(),
        description: "A small blue icegel".to_string(),
        ingredient_profile: small_blue_icegel_profile,
    };
    let small_red_icegel = Ingredient {
        name: "Small Red Icegel".to_string(),
        description: "A small red icegel".to_string(),
        ingredient_profile: small_red_icegel_profile,
    };
    let small_green_icegel = Ingredient {
        name: "Small Green Icegel".to_string(),
        description: "A small green icegel".to_string(),
        ingredient_profile: small_green_icegel_profile,
    };

    vec![
        (
            small_blue_icegel.clone(),
            blue_icegel_sprite.clone(),
            Transform::from_xyz(-400.0, 150.0, 1.0),
        ),
        (
            small_red_icegel.clone(),
            red_icegel_sprite.clone(),
            Transform::from_xyz(-200.0, 150.0, 1.0),
        ),
        (
            small_green_icegel.clone(),
            green_icegel_sprite.clone(),
            Transform::from_xyz(0.0, 150.0, 1.0),
        ),
        (
            small_blue_icegel.clone(),
            blue_icegel_sprite.clone(),
            Transform::from_xyz(-400.0, 50.0, 1.0),
        ),
        (
            small_red_icegel.clone(),
            red_icegel_sprite,
            Transform::from_xyz(-200.0, 50.0, 1.0),
        ),
        (
            small_green_icegel,
            green_icegel_sprite,
            Transform::from_xyz(0.0, 50.0, 1.0),
        ),
    ]
} 
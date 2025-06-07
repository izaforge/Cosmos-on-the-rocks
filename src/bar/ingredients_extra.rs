use bevy::prelude::*;

use crate::{
    bar::ingredient::{
        EffectCondition, Ingredient, IngredientProfile, IngredientTaste, PrimaryEffect,
        SecondaryEffect,
    },
    engine::asset_loader::ImageAssets,
};

pub fn get_other_ingredients(
    image_assets: &Res<ImageAssets>,
) -> Vec<(Ingredient, Sprite, Transform)> {
    let fizzion_mist_sprite = Sprite {
        image: image_assets.fizzion_mist.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let synth_vapor_sprite = Sprite {
        image: image_assets.synth_vapor.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let circuit_juice_sprite = Sprite {
        image: image_assets.circuit_juice.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let void_reserve_sprite = Sprite {
        image: image_assets.void_reserve.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };

    let fizzion_mist_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::Sour,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Aggresive(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let synth_vapor_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::Bitter,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Aggresive(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let circuit_juice_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::None,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Aggresive(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let void_reserve_profile = IngredientProfile {
        size: 0.1,
        taste: IngredientTaste::None,
        primary_effect: PrimaryEffect::Calming,
        secondary_effect: SecondaryEffect::Sedated(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let fizzion_mist = Ingredient {
        name: "Fizzion Mist".to_string(),
        description: "Fuzzy Drink".to_string(),
        ingredient_profile: fizzion_mist_profile,
    };

    let synth_vapor = Ingredient {
        name: "Synth Vapor".to_string(),
        description: "Vodka".to_string(),
        ingredient_profile: synth_vapor_profile,
    };

    let circuit_juice = Ingredient {
        name: "Circuit Juice".to_string(),
        description: "Gin".to_string(),
        ingredient_profile: circuit_juice_profile,
    };

    let void_reserve = Ingredient {
        name: "Void Reserve".to_string(),
        description: "Dark Matter".to_string(),
        ingredient_profile: void_reserve_profile,
    };

    vec![
        (
            fizzion_mist,
            fizzion_mist_sprite,
            Transform::from_xyz(-480.0, 50.0, 1.0),
        ),
        (
            synth_vapor,
            synth_vapor_sprite,
            Transform::from_xyz(-480.0, 200.0, 1.0),
        ),
        (
            circuit_juice,
            circuit_juice_sprite,
            Transform::from_xyz(-280.0, 200.0, 1.0),
        ),
        (
            void_reserve,
            void_reserve_sprite,
            Transform::from_xyz(-102.0, 200.0, 1.0),
        ),
    ]
}

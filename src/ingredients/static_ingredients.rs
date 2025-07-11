use bevy::prelude::*;

use crate::{
    engine::asset_loader::ImageAssets,
    ingredients::{
        EffectCondition, Ingredient, IngredientProfile, IngredientTaste, PrimaryEffect,
        SecondaryEffect,
    },
};

pub fn get_static_ingredients(
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
    let sweetflux_sprite = Sprite {
        image: image_assets.sweetflux.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let citraplasm_sprite = Sprite {
        image: image_assets.citraplasm.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };

    let fizzion_mist_profile = IngredientProfile {
        size: 10.,
        taste: IngredientTaste::Sour,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Aggresive(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let sweetflux_profile = IngredientProfile {
        size: 10.,
        taste: IngredientTaste::Sweet,
        primary_effect: PrimaryEffect::Healing,
        secondary_effect: SecondaryEffect::Euphoric(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let citraplasm_profile = IngredientProfile {
        size: 10.,
        taste: IngredientTaste::Citrus,
        primary_effect: PrimaryEffect::MindEnhancing,
        secondary_effect: SecondaryEffect::Hallucinogenic(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let synth_vapor_profile = IngredientProfile {
        size: 10.,
        taste: IngredientTaste::Bitter,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Aggresive(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let circuit_juice_profile = IngredientProfile {
        size: 10.,
        taste: IngredientTaste::Bitter,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Aggresive(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let void_reserve_profile = IngredientProfile {
        size: 10.,
        taste: IngredientTaste::Umami,
        primary_effect: PrimaryEffect::Calming,
        secondary_effect: SecondaryEffect::Sedated(EffectCondition {
            volume_needed: 40.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let fizzion_mist = Ingredient {
        name: "Fizzion Mist".to_string(),
        description:
            "A bubbly, sour liquid that crackles with energy, leaving a tingling sensation."
                .to_string(),
        ingredient_profile: fizzion_mist_profile,
    };

    let sweetflux = Ingredient {
        name: "Sweetflux".to_string(),
        description: "A luminous, sugary syrup that flows like liquid light, known for its calming properties.".to_string(),
        ingredient_profile: sweetflux_profile,
    };

    let citraplasm = Ingredient {
        name: "Citraplasm".to_string(),
        description:
            "A vibrant, zesty plasma with a sharp citrus bite, perfect for stimulating the mind."
                .to_string(),
        ingredient_profile: citraplasm_profile,
    };

    let synth_vapor = Ingredient {
        name: "Synth Vapor".to_string(),
        description: "A potent synthetic spirit, clear and almost tasteless, yet it carries a powerful, energizing kick.".to_string(),
        ingredient_profile: synth_vapor_profile,
    };

    let circuit_juice = Ingredient {
        name: "Circuit Juice".to_string(),
        description: "A sharp, botanical spirit with a metallic tang, designed to awaken the senses and enhance focus.".to_string(),
        ingredient_profile: circuit_juice_profile,
    };

    let void_reserve = Ingredient {
        name: "Void Reserve".to_string(),
        description: "A dense, inky fluid that absorbs light, offering a deep, earthy taste and a profound sense of calm.".to_string(),
        ingredient_profile: void_reserve_profile,
    };

    vec![
        (
            synth_vapor,
            synth_vapor_sprite,
            Transform::from_xyz(-480.0, 160.0, 1.0),
        ),
        (
            void_reserve,
            void_reserve_sprite,
            Transform::from_xyz(-280.0, 160.0, 1.0),
        ),
        (
            circuit_juice,
            circuit_juice_sprite,
            Transform::from_xyz(-80.0, 160.0, 1.0),
        ),
        (
            fizzion_mist,
            fizzion_mist_sprite,
            Transform::from_xyz(-480.0, 10.0, 1.0),
        ),
        (
            sweetflux,
            sweetflux_sprite,
            Transform::from_xyz(-280.0, 10.0, 1.0),
        ),
        (
            citraplasm,
            citraplasm_sprite,
            Transform::from_xyz(-80.0, 10.0, 1.0),
        ),
    ]
}

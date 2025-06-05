use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    animation::sprite_animation::SpriteAnimState,
    bar::{
        glass::{Glass, GlassShape},
        ingredient::{
            EffectCondition, Ingredient, IngredientProfile, IngredientTaste, PrimaryEffect,
            SecondaryEffect,
        },
    },
    engine::asset_loader::ImageAssets,
};

pub fn spawn_ingredients(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let icegels = get_ice_gels(image_assets, texture_atlases);
    let icegel_anim_state = SpriteAnimState {
        start_index: 0,
        end_index: 7,
        timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
    };
    for (ingredient, sprite, transform) in icegels {
        commands.spawn((ingredient, sprite, transform, icegel_anim_state.clone()));
    }
}

pub fn spawn_glass(mut commands: Commands, image_assets: Res<ImageAssets>) {

    let glass_sprite = Sprite {
        image: image_assets.wine_glass.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };

    let crafting_glass = Glass {
        capacity: 100.0,
        shape: GlassShape::Wine,
    };
    commands.spawn((
        crafting_glass,
        glass_sprite,
        Transform::from_translation(Vec3::new(200., 0., 1.)),
    ));
}


pub fn get_ice_gels(
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) -> Vec<(Ingredient, Sprite, Transform)> {
    let frame_size = UVec2::new(128, 128);
    let icegel_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        8,
        1,
        None,
        None,
    ));

    let blue_icegel_sprite = Sprite {
        image: image_assets.blue_icegel.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: icegel_layout_handle.clone(),
            index: 0,
        }),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let red_icegel_sprite = Sprite {
        image: image_assets.red_icegel.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: icegel_layout_handle.clone(),
            index: 0,
        }),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let green_icegel_sprite = Sprite {
        image: image_assets.green_icegel.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: icegel_layout_handle,
            index: 0,
        }),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };

    let blue_icegel_profile = IngredientProfile {
        size: 10.0,
        taste: IngredientTaste::None,
        primary_effect: PrimaryEffect::Calming,
        secondary_effect: SecondaryEffect::Sedated(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };
    let red_icegel_profile = IngredientProfile {
        size: 10.0,
        taste: IngredientTaste::Spicy,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Agitated(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };
    let green_icegel_profile = IngredientProfile {
        size: 10.0,
        taste: IngredientTaste::Sweet,
        primary_effect: PrimaryEffect::Healing,
        secondary_effect: SecondaryEffect::Euphoric(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };

    let blue_icegel = Ingredient {
        name: "Blue Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        id: Uuid::new_v4(),
        ingredient_profile: blue_icegel_profile,
    };
    let red_icegel_ingredient = Ingredient {
        name: "Red Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        id: Uuid::new_v4(),
        ingredient_profile: red_icegel_profile,
    };
    let green_icegel_ingredient = Ingredient {
        name: "Green Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        id: Uuid::new_v4(),
        ingredient_profile: green_icegel_profile,
    };
    vec![
        (
            blue_icegel,
            blue_icegel_sprite,
            Transform::from_xyz(-328.0, 0.0, 1.0),
        ),
        (
            red_icegel_ingredient,
            red_icegel_sprite,
            Transform::from_xyz(-228.0, 0.0, 1.0),
        ),
        (
            green_icegel_ingredient,
            green_icegel_sprite,
            Transform::from_xyz(-128.0, 0.0, 1.0),
        ),
    ]
}
use bevy::prelude::*;
use uuid::Uuid;

use crate::{bar::{
    glass::{Glass, GlassShape},
    ingredient::{
        EffectCondition, Ingredient, IngredientProfile, IngredientTaste, PrimaryEffect,
        SecondaryEffect,
    },
}, constants::WHITE, engine::asset_loader::ImageAssets};

pub fn spawn_ingredients(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let icegel_profile = IngredientProfile {
        size: 10.0,
        taste: IngredientTaste::None,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Euphoric(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };
    let icegel_ingredient = Ingredient {
        name: "Water".to_string(),
        description: "A basic ingredient for many drinks.".to_string(),
        id: Uuid::new_v4(),
        ingredient_profile: icegel_profile,
    };
    let frame_size = UVec2::new(128, 128);
    let icegel_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        2,
        1,
        None,
        None,
    ));

    let icegel_sprite = Sprite {
            image: image_assets.blue_icegel.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: icegel_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(96., 128.)),
            ..default()
        };

    commands.spawn((
        icegel_ingredient.clone(),
        icegel_sprite.clone(),
        Transform::from_translation(Vec3::new(200., 0., 1.)),
    ));
}

pub fn spawn_glass(mut commands: Commands) {
    let glass_sprite = Sprite {
        color: WHITE,
        custom_size: Some(Vec2::new(64., 64.)),
        ..default()
    };

    let crafting_glass = Glass {
        capacity: 100.0,
        shape: GlassShape::Whiskey,
    };
    commands.spawn((
        crafting_glass.clone(),
        glass_sprite.clone(),
        Transform::from_translation(Vec3::new(200., 0., 1.)),
    ));
}

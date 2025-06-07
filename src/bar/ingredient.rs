use bevy::prelude::*;

use crate::{
    animation::sprite_animation::SpriteAnimState,
    bar::{crafting::OnCraftingScreen, glass::Glass},
    engine::asset_loader::ImageAssets,
};

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform, SpriteAnimState)]
pub struct Ingredient {
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
    pub catalyst: Option<Entity>,
}

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
        commands
            .spawn((
                ingredient,
                sprite,
                transform,
                icegel_anim_state.clone(),
                Pickable::default(),
                OnCraftingScreen,
            ))
            .observe(
                |ev: Trigger<Pointer<Click>>,
                 mut glass_query: Query<&mut Glass>,
                 ingredient_query: Query<&Ingredient>| {
                    let ingredient_entity = ev.target();
                    for mut glass in glass_query.iter_mut() {
                        let ingredient_size = match ingredient_query.get(ingredient_entity) {
                            Ok(ingredient) => ingredient.ingredient_profile.size,
                            Err(_) => {
                                warn!("Clicked entity is not an ingredient!");
                                return;
                            }
                        };
                        if glass.get_current_volume() + ingredient_size < glass.capacity {
                            glass
                                .ingredients
                                .entry(ingredient_entity)
                                .and_modify(|v| *v += ingredient_size)
                                .or_insert(ingredient_size);
                            info!(
                                "Added ingredient {:#?} to glass with capacity {}",
                                glass.ingredients, glass.capacity
                            );
                        } else {
                            info!("Glass is full, cannot add more ingredients.");
                        }
                    }
                },
            );
    }
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
        size: 11.0,
        taste: IngredientTaste::Spicy,
        primary_effect: PrimaryEffect::Energizing,
        secondary_effect: SecondaryEffect::Agitated(EffectCondition {
            volume_needed: 90.0,
            catalyst: None,
        }),
        hazard: None,
    };
    let green_icegel_profile = IngredientProfile {
        size: 12.0,
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
        ingredient_profile: blue_icegel_profile.clone(),
    };
    let red_icegel_ingredient = Ingredient {
        name: "Red Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        ingredient_profile: red_icegel_profile.clone(),
    };
    let green_icegel_ingredient = Ingredient {
        name: "Green Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        ingredient_profile: green_icegel_profile.clone(),
    };

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
            red_icegel_sprite.clone(),
            Transform::from_xyz(-200.0, 50.0, 1.0),
        ),
        (
            small_green_icegel.clone(),
            green_icegel_sprite.clone(),
            Transform::from_xyz(0.0, 50.0, 1.0),
        ),
        (
            blue_icegel,
            blue_icegel_sprite,
            Transform::from_xyz(-500.0, -100.0, 1.0),
        ),
        (
            red_icegel_ingredient,
            red_icegel_sprite,
            Transform::from_xyz(-300.0, -100.0, 1.0),
        ),
        (
            green_icegel_ingredient,
            green_icegel_sprite,
            Transform::from_xyz(-100.0, -100.0, 1.0),
        ),
    ]
}

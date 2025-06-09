use bevy::{
    picking::{
        events::{Out, Over, Pointer, Pressed},
        prelude::Pickable,
    },
    prelude::*,
};

use crate::{
    animation::sprite_animation::SpriteAnimState,
    bar::{crafting::OnCraftingScreen, glass::Glass, ingredients_extra},
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum IngredientTaste {
    None,
    Sweet,
    Sour,
    Bitter,
    Citrus,
    Umami,
    Spicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    let icegels = get_ice_gels(&image_assets, texture_atlases);

    let other_ingredients = ingredients_extra::get_other_ingredients(&image_assets);

    for (ingredient, sprite, transform, anim_state) in icegels {
        commands
            .spawn((
                ingredient,
                sprite,
                transform,
                anim_state,
                Pickable::default(),
                OnCraftingScreen,
            ))
            .observe(
                |ev: Trigger<Pointer<Pressed>>,
                 mut glass_query: Query<&mut Glass>,
                 ingredient_query: Query<&Ingredient>| {
                    let ingredient_entity = ev.target();
                    for mut glass in glass_query.iter_mut() {
                        let (ingredient_size, ingredient_taste, ingredient_effect) = match ingredient_query.get(ingredient_entity) {
                            Ok(ingredient) => (ingredient.ingredient_profile.size, ingredient.ingredient_profile.taste, ingredient.ingredient_profile.primary_effect),
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
                                "Added ingredient {:#?} to glass with capacity {} current taste {:#?}",
                                glass.ingredients, glass.capacity, glass.taste
                            );
                            glass
                                .taste
                                .entry(ingredient_taste)
                                .and_modify(|v| *v += ingredient_size)
                                .or_insert(ingredient_size);
                            glass
                                .effect
                                .entry(ingredient_effect)
                                .and_modify(|v| *v += ingredient_size)
                                .or_insert(ingredient_size);
                        } else {
                            info!("Glass is full, cannot add more ingredients.");
                        }
                    }
                },
            )
            .observe(
                |ev: Trigger<Pointer<Over>>, ingredient_query: Query<&Ingredient>, asset_server: Res<AssetServer>, mut commands: Commands,| {
                    if let Ok(ingredient) = ingredient_query.get(ev.target()) {
                        commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new(format!("{} : {} Taste {:#?}", ingredient.name, ingredient.description, ingredient.ingredient_profile.taste)),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/DigitTech14-Regular.ttf"),
            font_size: 10.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Right),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));

                    }
                },
            )
            .observe(|_: Trigger<Pointer<Out>>| {
                // Hover end - tooltip will be handled by the UI system
            });
    }

    for (ingredient, sprite, transform) in other_ingredients {
        commands
            .spawn((
                ingredient,
                sprite,
                transform,
                Pickable::default(),
                OnCraftingScreen,
            ))
            .observe(
                |ev: Trigger<Pointer<Pressed>>,
                 mut glass_query: Query<&mut Glass>,
                 ingredient_query: Query<&Ingredient>| {
                    let ingredient_entity = ev.target();
                    for mut glass in glass_query.iter_mut() {
                        let (ingredient_size, ingredient_taste, ingredient_effect) = match ingredient_query.get(ingredient_entity) {
                            Ok(ingredient) => (ingredient.ingredient_profile.size, ingredient.ingredient_profile.taste, ingredient.ingredient_profile.primary_effect),
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
                                "Added ingredient {:#?} to glass with capacity {} current taste {:#?}",
                                glass.ingredients, glass.capacity, glass.taste
                            );
                            glass
                                .taste
                                .entry(ingredient_taste)
                                .and_modify(|v| *v += ingredient_size)
                                .or_insert(ingredient_size);
                            glass
                                .effect
                                .entry(ingredient_effect)
                                .and_modify(|v| *v += ingredient_size)
                                .or_insert(ingredient_size);
                        } else {
                            info!("Glass is full, cannot add more ingredients.");
                        }
                    }
                },
            );
    }
}

pub fn get_ice_gels(
    image_assets: &Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) -> Vec<(Ingredient, Sprite, Transform, SpriteAnimState)> {
    let frame_size = UVec2::new(128, 128);
    let icegel_layout_handle =
        texture_atlases.add(TextureAtlasLayout::from_grid(frame_size, 8, 1, None, None));
    let icegel_anim_state = SpriteAnimState {
        start_index: 0,
        end_index: 7,
        timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
    };
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
        taste: IngredientTaste::Umami,
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
        taste: IngredientTaste::Bitter,
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
        ingredient_profile: blue_icegel_profile,
    };
    let red_icegel_ingredient = Ingredient {
        name: "Red Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        ingredient_profile: red_icegel_profile,
    };
    let green_icegel_ingredient = Ingredient {
        name: "Green Icegel".to_string(),
        description: "Cools down drinks".to_string(),
        ingredient_profile: green_icegel_profile,
    };

    vec![
        (
            blue_icegel,
            blue_icegel_sprite,
            Transform::from_xyz(-450.0, -200.0, 1.1),
            icegel_anim_state.clone(),
        ),
        (
            red_icegel_ingredient,
            red_icegel_sprite,
            Transform::from_xyz(-400.0, -200.0, 1.0),
            icegel_anim_state.clone(),
        ),
        (
            green_icegel_ingredient,
            green_icegel_sprite,
            Transform::from_xyz(-350.0, -200.0, 1.1),
            icegel_anim_state.clone(),
        ),
    ]
}

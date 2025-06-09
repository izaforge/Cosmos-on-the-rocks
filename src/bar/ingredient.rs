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
    constants::{BUTTON_BORDER, NORMAL_BUTTON, TEXT_COLOR},
    engine::asset_loader::ImageAssets,
    ui::ingredient_tooltip::IngredientTooltip,
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

#[derive(Component)]
pub struct GlassFullIndicator;

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
                 ingredient_query: Query<&Ingredient>,
                 asset_server: Res<AssetServer>,
                 mut commands: Commands| {
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
                            commands.spawn((
                                GlassFullIndicator,
                                Text::new("Glass is Full!"),
                                Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
                                BorderColor(Color::srgb(1.0, 0.0, 0.0)),
                                BorderRadius::ZERO,
                                BackgroundColor(Color::srgb(0.8, 0.1, 0.1)),
                                TextFont {
                                    font: asset_server.load("fonts/Nasa21.ttf"),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(20.0),
                                    left: Val::Px(20.0),
                                    align_items: AlignItems::FlexStart,
                                    justify_content: JustifyContent::FlexStart,
                                    ..Default::default()
                                },
                                GlassFullTimer(Timer::from_seconds(2.0, TimerMode::Once)),
                            ));
                        }
                    }
                },
            )
            .observe(
                |ev: Trigger<Pointer<Over>>, ingredient_query: Query<&Ingredient>, asset_server: Res<AssetServer>, mut commands: Commands,| {
                    if let Ok(ingredient) = ingredient_query.get(ev.target()) {
                        commands.spawn((
                            IngredientTooltip,
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new(format!(
                    "{} : {}\nTaste: {:#?}\nEffects: {:#?}",
                    ingredient.name,
                    ingredient.description,
                    ingredient.ingredient_profile.taste,
                    ingredient.ingredient_profile.primary_effect
                )),
                Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
                BorderColor(BUTTON_BORDER),
                BorderRadius::ZERO,
                BackgroundColor(NORMAL_BUTTON),
                TextFont {
            font: asset_server.load("fonts/Nasa21.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor (TEXT_COLOR),
        // Set the style of the Node itself.
            Node {
                    position_type: PositionType::Absolute,
    top: Val::Px(20.0),
    right: Val::Px(20.0),
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    ..Default::default()
        },
    ));

                    }
                },
            )
            .observe(|_: Trigger<Pointer<Out>>, mut commands: Commands, tooltip_query: Query<Entity, With<IngredientTooltip>>| {
                for entity in tooltip_query.iter() {
                        commands.entity(entity).despawn();
                    }
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
                 ingredient_query: Query<&Ingredient>,
                 asset_server: Res<AssetServer>,
                 mut commands: Commands| {
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
                            commands.spawn((
                                GlassFullIndicator,
                                Text::new("Glass is Full!"),
                                Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
                                BorderColor(Color::srgb(1.0, 0.0, 0.0)),
                                BorderRadius::ZERO,
                                BackgroundColor(Color::srgb(0.8, 0.1, 0.1)),
                                TextFont {
                                    font: asset_server.load("fonts/Nasa21.ttf"),
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(20.0),
                                    left: Val::Px(20.0),
                                    align_items: AlignItems::FlexStart,
                                    justify_content: JustifyContent::FlexStart,
                                    ..Default::default()
                                },
                                GlassFullTimer(Timer::from_seconds(2.0, TimerMode::Once)),
                            ));
                        }
                    }
                },
            ).observe(
                |ev: Trigger<Pointer<Over>>, ingredient_query: Query<&Ingredient>, asset_server: Res<AssetServer>, mut commands: Commands,| {
                    if let Ok(ingredient) = ingredient_query.get(ev.target()) {
                        commands.spawn((
                            IngredientTooltip,
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new(format!(
                    "{} : {}\nTaste: {:#?}\nEffects: {:#?}",
                    ingredient.name,
                    ingredient.description,
                    ingredient.ingredient_profile.taste,
                    ingredient.ingredient_profile.primary_effect
                )),
                Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
                BorderColor(BUTTON_BORDER),
                BorderRadius::ZERO,
                BackgroundColor(NORMAL_BUTTON),
                TextFont {
            font: asset_server.load("fonts/Nasa21.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor (TEXT_COLOR),
        // Set the style of the Node itself.
            Node {
                    position_type: PositionType::Absolute,
    top: Val::Px(20.0),
    right: Val::Px(20.0),
    align_items: AlignItems::FlexStart,
    justify_content: JustifyContent::FlexStart,
    ..Default::default()
        },
    ));

                    }
                },
            )
            .observe(|_: Trigger<Pointer<Out>>, mut commands: Commands, tooltip_query: Query<Entity, With<IngredientTooltip>>| {
                for entity in tooltip_query.iter() {
                        commands.entity(entity).despawn();
                    }
            });
    }
}

// New system to despawn the GlassFullIndicator
pub fn despawn_glass_full_indicator(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GlassFullTimer), With<GlassFullIndicator>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct GlassFullTimer(Timer);

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

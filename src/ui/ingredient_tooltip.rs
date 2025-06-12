use bevy::prelude::*;

use crate::{
    bar::glass::Glass,
    constants::{BUTTON_BORDER, NORMAL_BUTTON, TEXT_COLOR},
    ingredients::Ingredient, ui::crafting_menu::GlassDetailsUI,
};

#[derive(Component)]
pub struct IngredientTooltip;

pub fn ingredient_pressed(
    ev: Trigger<Pointer<Pressed>>,
    mut glass_query: Query<&mut Glass>,
    ingredient_query: Query<&Ingredient>,
    mut ui_text: Query<&mut Text, With<GlassDetailsUI>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let ingredient_entity = ev.target();
    for mut glass in glass_query.iter_mut() {
        let (ingredient_size, ingredient_taste, ingredient_effect) =
            match ingredient_query.get(ingredient_entity) {
                Ok(ingredient) => (
                    ingredient.ingredient_profile.size,
                    ingredient.ingredient_profile.taste,
                    ingredient.ingredient_profile.primary_effect,
                ),
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
            for mut text in ui_text.iter_mut() {
                text.0 = format!(
                    "Volume: {:.1} / {:.1}\nTaste: {:#?}\nEffects: {:#?}",
                    glass.get_current_volume(),
                    glass.capacity,
                    glass.taste,
                    glass.effect,
                );
            }
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
}

pub fn ingredient_hover(
    ev: Trigger<Pointer<Over>>,
    ingredient_query: Query<&Ingredient>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
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
            TextColor(TEXT_COLOR),
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
}

pub fn ingredient_hover_out(
    _: Trigger<Pointer<Out>>,
    mut commands: Commands,
    tooltip_query: Query<Entity, With<IngredientTooltip>>,
) {
    for entity in tooltip_query.iter() {
        commands.entity(entity).despawn();
    }
}

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
pub struct GlassFullTimer(pub Timer);

#[derive(Component)]
pub struct GlassFullIndicator;

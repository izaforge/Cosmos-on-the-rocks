use bevy::prelude::*;

use crate::{
    animation::{sprite_animation::animate_spite, AnimationEvent},
    bar::{bar_counter::spawn_crafting_area, glass::spawn_glass, ingredient::spawn_ingredients},
    customers::spawn_bartender,
    engine::{audio_controller::play_crafting_bg, GameState},
};

#[derive(Component)]
pub struct OnCraftingScreen;

#[derive(Component)]
pub enum CraftButtons {
    Craft,
    Reset,
}

/// Component for ingredient tooltip UI
#[derive(Component)]
pub struct IngredientTooltip;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Crafting),
            (
                spawn_ingredients,
                spawn_glass,
                spawn_bartender,
                play_crafting_bg,
                spawn_crafting_area,
            ),
        )
        .add_systems(
            Update, 
            (
                animate_spite,
                handle_ingredient_hover,
            ).run_if(in_state(GameState::Crafting))
        )
        .add_systems(OnExit(GameState::Crafting), cleanup_crafting)
        .add_event::<AnimationEvent>();
    }
}

/// System to handle ingredient hover tooltips
fn handle_ingredient_hover(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &crate::bar::ingredient::Ingredient), With<crate::bar::ingredient::Ingredient>>,
    tooltip_query: Query<Entity, With<IngredientTooltip>>,
    mut tooltip_data: Local<Option<Entity>>, // Track which ingredient currently has tooltip
) {
    let mut current_hovered = None;
    
    // Find currently hovered ingredient
    for (entity, interaction, _) in interaction_query.iter() {
        if matches!(*interaction, Interaction::Hovered) {
            current_hovered = Some(entity);
            break;
        }
    }
    
    // If the hovered ingredient changed, update tooltip
    if *tooltip_data != current_hovered {
        // Clean up existing tooltips
        for tooltip_entity in tooltip_query.iter() {
            commands.entity(tooltip_entity).despawn_recursive();
        }
        
        // Create new tooltip if hovering over an ingredient
        if let Some(hovered_entity) = current_hovered {
            if let Ok((_, _, ingredient)) = interaction_query.get(hovered_entity) {
                // Calculate effect strength based on ingredient size (normalized to 0-10 scale)
                let effect_strength = ((ingredient.ingredient_profile.size / 10.0).min(1.0) * 10.0) as u8;
                
                // Create tooltip
                commands
                    .spawn((
                        IngredientTooltip,
                        OnCraftingScreen,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(50.0),
                            top: Val::Px(50.0),
                            padding: UiRect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            max_width: Val::Px(250.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
                        BorderRadius::all(Val::Px(5.0)),
                        ZIndex(1000),
                    ))
                    .with_children(|parent| {
                        // Ingredient name
                        parent.spawn((
                            Text::new(&ingredient.name),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                        
                        // Description
                        parent.spawn((
                            Text::new(&ingredient.description),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                        
                        // Primary effect
                        parent.spawn((
                            Text::new(format!("Primary Effect: {:?} (+{})", 
                                ingredient.ingredient_profile.primary_effect, 
                                effect_strength)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 1.0, 0.5)),
                        ));
                        
                        // Taste
                        if ingredient.ingredient_profile.taste != crate::bar::ingredient::IngredientTaste::None {
                            parent.spawn((
                                Text::new(format!("Taste: {:?}", ingredient.ingredient_profile.taste)),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 1.0)),
                            ));
                        }
                        
                        // Volume
                        parent.spawn((
                            Text::new(format!("Volume: {:.1}", ingredient.ingredient_profile.size)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                    });
            }
        }
        
        *tooltip_data = current_hovered;
    }
}

pub fn cleanup_crafting(mut commands: Commands, query: Query<Entity, With<OnCraftingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

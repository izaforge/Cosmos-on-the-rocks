use bevy::prelude::*;
use bevy::log::warn;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use crate::bar::ingredient::PrimaryEffect;
use std::collections::HashMap;

use crate::{
    customers::{OnCustomerScreen, cleanup_customer},
    engine::GameState,
};

pub struct DialogPlugin;

/// Resource to specify which dialogue node to start next
#[derive(Resource)]
pub struct NextDialogueNode(pub String);

/// Resource to store effect values for dialogue branching
#[derive(Resource, Default)]
pub struct PatronEffects {
    pub effects: HashMap<PrimaryEffect, u8>, // Values range from 0-10
}

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .init_resource::<PatronEffects>()
        .add_systems(
            OnEnter(GameState::CustomerInteraction),
            spawn_dialogue_runner,
        )
        .add_systems(Update, (handle_drink_effects, convert_drink_to_effects))
        .add_systems(OnExit(GameState::CustomerInteraction), cleanup_customer);
    }
}

pub fn spawn_dialogue_runner(
    mut commands: Commands, 
    project: Res<YarnProject>,
    next_node: Option<Res<NextDialogueNode>>,
    existing_dialogue: Query<Entity, With<DialogueRunner>>,
    existing_ui: Query<Entity, With<OnCustomerScreen>>,
    patron_effects: Res<PatronEffects>,
) {
    // Clean up any existing dialogue runners and UI to prevent conflicts
    for entity in existing_dialogue.iter() {
        commands.entity(entity).despawn();
    }
    
    for entity in existing_ui.iter() {
        commands.entity(entity).despawn();
    }
    
    println!("Spawning dialogue runner with project: {:?}", project);
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    
    // Register the system and get its SystemId
    let set_game_state_id = commands.register_system(set_game_state);
    
    // Set variables for effect values in dialogue
    let calming_value = patron_effects.effects.get(&PrimaryEffect::Calming).unwrap_or(&0);
    let energizing_value = patron_effects.effects.get(&PrimaryEffect::Energizing).unwrap_or(&0);
    let mind_enhancing_value = patron_effects.effects.get(&PrimaryEffect::MindEnhancing).unwrap_or(&0);
    let courage_value = patron_effects.effects.get(&PrimaryEffect::CourageBoosting).unwrap_or(&0);
    let truth_value = patron_effects.effects.get(&PrimaryEffect::TruthInducing).unwrap_or(&0);
    let healing_value = patron_effects.effects.get(&PrimaryEffect::Healing).unwrap_or(&0);
    
    // Register the custom command that allows changing the game state from dialogue
    dialogue_runner
        .commands_mut()
        .add_command("set_game_state", set_game_state_id);
    
    // Set variables for dialogue to use
    let mut storage = dialogue_runner.variable_storage_mut();
    let mut variables = storage.variables();
    variables.insert("$calming_effect".to_string(), (*calming_value).into());
    variables.insert("$energizing_effect".to_string(), (*energizing_value).into());
    variables.insert("$mind_enhancing_effect".to_string(), (*mind_enhancing_value).into());
    variables.insert("$courage_effect".to_string(), (*courage_value).into());
    variables.insert("$truth_effect".to_string(), (*truth_value).into());
    variables.insert("$healing_effect".to_string(), (*healing_value).into());
    
    // Print debug header
    println!("============================================");
    println!("Current drink effects: {:?}", patron_effects.effects);
    println!("Checking for dialogue nodes");
    
    // Check if we have a specific node to start
    if let Some(next_node) = next_node {
        let node_exists = project.headers_for_node(&next_node.0).is_some();
        if node_exists {
            dialogue_runner.start_node(&next_node.0);
            println!("✅ Starting specified node: {}", next_node.0);
            commands.remove_resource::<NextDialogueNode>(); // Remove the resource after use
            commands.spawn((dialogue_runner, OnCustomerScreen));
            return;
        }
    }
    
    // Default flow if no specific node is requested
    let zara_dialogue = project.headers_for_node("ZaraDialogue");
    let patron_dialogue = project.headers_for_node("PatronDialogue");
    let bartender_dialogue = project.headers_for_node("BartenderMonologue");
    
    println!("ZaraDialogue: {}", if zara_dialogue.is_some() { "Found ✅" } else { "Not found ❌" });
    println!("PatronDialogue: {}", if patron_dialogue.is_some() { "Found ✅" } else { "Not found ❌" });
    println!("BartenderMonologue: {}", if bartender_dialogue.is_some() { "Found ✅" } else { "Not found ❌" });
    println!("============================================");
    
    // First try to use Zara's dialogue
    if zara_dialogue.is_some() {
        dialogue_runner.start_node("ZaraDialogue");
        println!("✅ SUCCESS: Starting ZaraDialogue node from zara.yarn");
    } 
    // Fall back to the patron dialogue
    else if patron_dialogue.is_some() {
        dialogue_runner.start_node("PatronDialogue");
        println!("Starting PatronDialogue node");
    } 
    // Last resort is the bartender monologue
    else if bartender_dialogue.is_some() {
        dialogue_runner.start_node("BartenderMonologue");
        println!("Starting BartenderMonologue node");
    } 
    else {
        println!("❌ ERROR: No dialogue node found!");
    }

    commands.spawn((dialogue_runner, OnCustomerScreen));
}

// Command function to handle game state changes
fn set_game_state(In(state_name): In<String>, world: &mut World) {
    let mut next_state = world.resource_mut::<NextState<GameState>>();
    match state_name.as_str() {
        "MainMenu" => next_state.set(GameState::MainMenu),
        "Loading" => next_state.set(GameState::Loading),
        "Settings" => next_state.set(GameState::Settings),
        "CustomerInteraction" => next_state.set(GameState::CustomerInteraction),
        "Crafting" => next_state.set(GameState::Crafting),
        "EndNight" => next_state.set(GameState::EndNight),
        _ => warn!("Unknown GameState: {}", state_name),
    }
}

/// Component for a drink that can affect customers based on primary effects
#[derive(Component)]
pub struct EffectDrink {
    pub effects: HashMap<PrimaryEffect, u8>, // Values range from 0-10
    pub target_patron: Option<Entity>,
}

/// System to handle drink effects on patrons
pub fn handle_drink_effects(
    mut commands: Commands,
    drinks: Query<(Entity, &EffectDrink)>,
    mut patron_effects: ResMut<PatronEffects>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Just for testing - press T to apply a test drink effect
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        // Create a test drink
        let mut effects = HashMap::new();
        effects.insert(PrimaryEffect::Energizing, 8);
        effects.insert(PrimaryEffect::MindEnhancing, 5);
        
        commands.spawn(EffectDrink {
            effects,
            target_patron: None, // Apply to current patron
        });
        
        info!("Serving test drink to patron!");
    }
    
    // Press E to manually add effects for testing
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        patron_effects.effects.insert(PrimaryEffect::Energizing, 8);
        patron_effects.effects.insert(PrimaryEffect::TruthInducing, 6);
        patron_effects.effects.insert(PrimaryEffect::MindEnhancing, 4);
        info!("🧪 TEST: Manually added effects: {:?}", patron_effects.effects);
    }
    
    // Process all effect drinks
    for (drink_entity, drink) in drinks.iter() {
        // If no specific target, apply to the current patron effects resource
        if drink.target_patron.is_none() {
            info!("🍸 Processing drink effects: {:?}", drink.effects);
            
            // Apply or update each effect
            for (effect, value) in &drink.effects {
                let current_value = *patron_effects.effects.get(effect).unwrap_or(&0);
                let new_value = (current_value + value).min(10);
                patron_effects.effects.insert(effect.clone(), new_value);
                
                info!("✅ Applied {:?} effect: {} -> {}", effect, current_value, new_value);
            }
            
            info!("🎯 Final patron effects: {:?}", patron_effects.effects);
            
            // Remove the drink entity after it's been applied
            commands.entity(drink_entity).despawn();
        }
    }
}

/// System to convert a crafted drink into effects
pub fn convert_drink_to_effects(
    mut commands: Commands,
    drinks: Query<(Entity, &crate::bar::drinks::Drink), Added<crate::bar::drinks::Drink>>,
    ingredients: Query<&crate::bar::ingredient::Ingredient>,
) {
    for (drink_entity, drink) in drinks.iter() {
        let mut effects = HashMap::new();
        
        // Extract primary effects from ingredients
        for (ingredient_entity, amount) in &drink.ingredients {
            if let Ok(ingredient) = ingredients.get(*ingredient_entity) {
                let effect = ingredient.ingredient_profile.primary_effect.clone();
                // Calculate effect strength based on amount (normalized to 0-10 scale)
                let effect_strength = ((amount / 10.0).min(1.0) * 10.0) as u8;
                
                // Add or increase the effect value
                let current_value = effects.get(&effect).unwrap_or(&0);
                let new_value = (*current_value + effect_strength).min(10);
                effects.insert(effect.clone(), new_value);
                
                info!("Ingredient {} contributes {:?} effect with strength {}", 
                      ingredient.name, effect, effect_strength);
            }
        }
        
        // Fallback: If no ingredients found, use taste-based effects (for backwards compatibility)
        if effects.is_empty() {
            match drink.taste.primary_taste {
                crate::bar::ingredient::IngredientTaste::Sweet => {
                    effects.insert(PrimaryEffect::Healing, 7);
                    effects.insert(PrimaryEffect::Calming, 3);
                },
                crate::bar::ingredient::IngredientTaste::Sour => {
                    effects.insert(PrimaryEffect::Energizing, 8);
                    effects.insert(PrimaryEffect::MindEnhancing, 2);
                },
                crate::bar::ingredient::IngredientTaste::Bitter => {
                    effects.insert(PrimaryEffect::TruthInducing, 9);
                },
                crate::bar::ingredient::IngredientTaste::Spicy => {
                    effects.insert(PrimaryEffect::CourageBoosting, 8);
                    effects.insert(PrimaryEffect::Energizing, 4);
                },
                _ => {
                    // Default effect for other tastes
                    effects.insert(PrimaryEffect::Calming, 5);
                }
            }
        }
        
        info!("Created effect drink with effects: {:?}", effects);
        
        // Spawn the effect drink
        commands.spawn(EffectDrink {
            effects,
            target_patron: None, // Apply to current patron
        });
        
        // Remove the original drink
        commands.entity(drink_entity).despawn();
    }
}
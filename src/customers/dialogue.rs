use bevy::prelude::*;
use bevy::log::warn;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use crate::dialogue::patrons::{Happiness, Sadness, Anger, Patron};

use crate::{
    customers::{OnCustomerScreen, cleanup_customer},
    engine::GameState,
};

pub struct DialogPlugin;

/// Resource to specify which dialogue node to start next
#[derive(Resource)]
pub struct NextDialogueNode(pub String);

/// Resource to track if we should auto-press the 1 key
#[derive(Resource, Default)]
struct AutoSelectOption(bool);

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .add_systems(
            OnEnter(GameState::CustomerInteraction),
            spawn_dialogue_runner,
        )
        .add_systems(OnExit(GameState::CustomerInteraction), cleanup_customer);
    }
}

// System to automatically select the first option by simulating pressing "1"
fn auto_select_first_option(
    dialogue_runners: Query<&DialogueRunner>,
    mut auto_select: ResMut<AutoSelectOption>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Only proceed if there's an active dialogue runner
    if !dialogue_runners.is_empty() && !auto_select.0 {
        // Set the flag to true to prevent multiple keypresses
        auto_select.0 = true;
        
        // Add a small delay to ensure UI is ready
        // Then simulate pressing "1" key
        info!("Auto-selecting first option");
        keyboard.press(KeyCode::Digit1);
    }
}

pub fn spawn_dialogue_runner(
    mut commands: Commands, 
    project: Res<YarnProject>,
    next_node: Option<Res<NextDialogueNode>>,
    existing_dialogue: Query<Entity, With<DialogueRunner>>,
    existing_ui: Query<Entity, With<OnCustomerScreen>>,
) {
    // Reset auto-select flag when spawning a new dialogue
    commands.insert_resource(AutoSelectOption(false));
    
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
    
    // Register the custom command that allows changing the game state from dialogue
    dialogue_runner
        .commands_mut()
        .add_command("set_game_state", set_game_state_id);
    
    // Print debug header
    println!("============================================");
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

/// Component for a drink that can affect emotions
#[derive(Component)]
pub struct EmotionDrink {
    pub happiness_effect: i16,
    pub sadness_effect: i16,
    pub anger_effect: i16,
    pub target_patron: Option<Entity>,
}

/// System to handle drink effects on patron emotions
pub fn handle_drink_effects(
    mut commands: Commands,
    drinks: Query<(Entity, &EmotionDrink)>,
    mut patrons: Query<(Entity, &Patron, &mut Happiness, &mut Sadness, &mut Anger)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Just for testing - press T to apply a drink effect to the first patron
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        // For testing, just find the first patron
        if let Some((entity, _, _, _, _)) = patrons.iter().next() {
            // Create a test drink
            commands.spawn(EmotionDrink {
                happiness_effect: 10,
                sadness_effect: -5,
                anger_effect: -15,
                target_patron: Some(entity),
            });
            
            info!("Serving test drink to patron!");
        }
    }
    
    // Process all emotion drinks and apply effects
    for (drink_entity, drink) in drinks.iter() {
        // If target patron is specified, use that
        if let Some(target) = drink.target_patron {
            if let Ok((_, patron, mut happiness, mut sadness, mut anger)) = patrons.get_mut(target) {
                // Apply emotion changes
                happiness.value = ((happiness.value as i16) + drink.happiness_effect).clamp(0, 100) as u8;
                sadness.value = ((sadness.value as i16) + drink.sadness_effect).clamp(0, 100) as u8;
                anger.value = ((anger.value as i16) + drink.anger_effect).clamp(0, 100) as u8;
                
                info!(
                    "Applied drink to {}: Happiness: {}, Sadness: {}, Anger: {}", 
                    patron.name, happiness.value, sadness.value, anger.value
                );
                
                // Remove the drink entity after it's been applied
                commands.entity(drink_entity).despawn();
            }
        } 
        // If no target specified, try to find Zara
        else {
            // Try to find Zara by name
            for (_entity, patron, mut happiness, mut sadness, mut anger) in patrons.iter_mut() {
                if patron.name == "Zara" {
                    // Apply emotion changes
                    happiness.value = ((happiness.value as i16) + drink.happiness_effect).clamp(0, 100) as u8;
                    sadness.value = ((sadness.value as i16) + drink.sadness_effect).clamp(0, 100) as u8;
                    anger.value = ((anger.value as i16) + drink.anger_effect).clamp(0, 100) as u8;
                    
                    info!(
                        "Applied drink to Zara: Happiness: {}, Sadness: {}, Anger: {}", 
                        happiness.value, sadness.value, anger.value
                    );
                    
                    // Remove the drink entity after it's been applied
                    commands.entity(drink_entity).despawn();
                    break; // Stop after finding Zara
                }
            }
        }
    }
}
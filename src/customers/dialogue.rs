use bevy::prelude::*;
use bevy::log::warn;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{customers::OnCustomerScreen, engine::GameState};

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .add_systems(OnEnter(GameState::CustomerInteraction), spawn_dialogue_runner);
    }
}

pub fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
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
    
    // First try to use Zara's dialogue
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
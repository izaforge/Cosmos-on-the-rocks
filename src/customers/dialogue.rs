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
    
    // Start with the patron dialogue instead of bartender monologue
    // Check if nodes exist by looking at their headers
    if project.headers_for_node("PatronDialogue").is_some() {
        dialogue_runner.start_node("PatronDialogue");
        println!("Starting PatronDialogue node");
    } else if project.headers_for_node("BartenderMonologue").is_some() {
        dialogue_runner.start_node("BartenderMonologue");
        println!("Starting BartenderMonologue node");
    } else {
        println!("No dialogue node found!");
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
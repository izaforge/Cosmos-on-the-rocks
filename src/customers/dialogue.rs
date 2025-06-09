use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{customers::OnCustomerScreen, engine::GameState};

#[derive(Resource, Default)]
pub struct DialogueState {
    pub bartender_monologue_played: bool,
    pub bartender_drink_finished: bool,
}

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/on_the_rocks.yarn")),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .init_resource::<DialogueState>()
        .add_systems(OnEnter(GameState::Dialogues), spawn_dialogue_runner);
    }
}

pub fn spawn_dialogue_runner(
    mut commands: Commands, 
    project: Res<YarnProject>,
    mut dialogue_state: ResMut<DialogueState>
) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.commands_mut().add_command(
        "change_gamestate",
        commands.register_system(change_gamestate),
    );
    
    // Choose starting node based on dialogue state
    let starting_node = if !dialogue_state.bartender_monologue_played {
        dialogue_state.bartender_monologue_played = true;
        "BartenderMonologue"
    } else if !dialogue_state.bartender_drink_finished {
        dialogue_state.bartender_drink_finished = true;
        "BartenderAfterDrink"
    } else {
        "ZaraDialogue"
    };
    
    dialogue_runner.start_node(starting_node);
    commands.spawn((dialogue_runner, OnCustomerScreen));
}

fn change_gamestate(In(state): In<String>, mut game_state: ResMut<NextState<GameState>>) {
    match state.as_str() {
        "Crafting" => game_state.set(GameState::Crafting),
        "End" => game_state.set(GameState::EndNight),
        "Menu" => game_state.set(GameState::MainMenu),
        _ => println!("Unknown game state: {}", state),
    }
}

use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{
    customers::{OnCustomerScreen},
    engine::GameState,
};

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/on_the_rocks.yarn")),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .add_systems(OnEnter(GameState::Dialogues), spawn_dialogue_runner);
    }
}

pub fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.commands_mut().add_command(
        "change_gamestate",
        commands.register_system(change_gamestate),
    );
    dialogue_runner.start_node("BartenderMonologue");
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

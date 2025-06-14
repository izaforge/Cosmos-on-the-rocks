use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{customers::OnCustomerScreen, engine::GameState};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum DialogueState {
    #[default]
    BartenderMonologue,
    CarlEnters,
    ZaraEnters,
    CodaEnters,
}

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/on_the_rocks.yarn")),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .init_state::<DialogueState>()
        .add_systems(OnEnter(GameState::Dialogues), spawn_dialogue_runner);
    }
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    dialogue_state: Res<State<DialogueState>>,
) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.commands_mut().add_command(
        "change_gamestate",
        commands.register_system(change_gamestate),
    );

    // Choose starting node based on dialogue state
    let starting_node = match dialogue_state.get() {
        DialogueState::BartenderMonologue => "BartenderMonologue",
        DialogueState::CarlEnters => "CarlEnters",
        DialogueState::ZaraEnters => "ZaraEnters",
        DialogueState::CodaEnters => "CodaEnters",
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

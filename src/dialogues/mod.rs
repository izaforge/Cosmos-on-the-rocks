use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{customers::OnCustomerScreen, engine::GameState, ui::crafting_ui::DrinkSprite};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum DialogueState {
    #[default]
    BartenderMonologue,
    //BartenderDrinks,
    CarlEnters,
    //CarlDrinks,
    ZaraEnters,
    //ZaraDrinks,
    CodaEnters,
    //CodaDrinks,
    MysteryEnters,
    //MysteryDrinks,
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
    dialogue_runner
        .commands_mut()
        .add_command(
            "change_gamestate",
            commands.register_system(change_gamestate),
        )
        .add_command(
            "change_dialog_state",
            commands.register_system(change_dialog_state),
        )
        .add_command("consume_drink", commands.register_system(consume_drink));

    // Choose starting node based on dialogue state
    let node = match dialogue_state.get() {
        DialogueState::BartenderMonologue => "BartenderMonologue",
        DialogueState::CarlEnters => "CarlEnters",
        DialogueState::ZaraEnters => "ZaraEnters",
        DialogueState::CodaEnters => "CodaEnters",
        DialogueState::MysteryEnters => "MysteryEnters",
    };

    dialogue_runner.start_node(node);
    commands.spawn((dialogue_runner, OnCustomerScreen));
}

fn change_gamestate(In(state): In<String>, mut game_state: ResMut<NextState<GameState>>) {
    info!("Changing game state to: {}", state);
    match state.as_str() {
        "Crafting" => game_state.set(GameState::Crafting),
        "End" => game_state.set(GameState::EndNight),
        "Menu" => game_state.set(GameState::MainMenu),
        _ => println!("Unknown game state: {}", state),
    }
}

fn change_dialog_state(In(state): In<String>, mut dialog_state: ResMut<NextState<DialogueState>>) {
    info!("Changing dialogue state to: {}", state);
    match state.as_str() {
        "Carl" => dialog_state.set(DialogueState::CarlEnters),
        "Zara" => dialog_state.set(DialogueState::ZaraEnters),
        "Coda" => dialog_state.set(DialogueState::CodaEnters),
        "Mystery" => dialog_state.set(DialogueState::MysteryEnters),
        _ => println!("Unknown dialogue state: {}", state),
    }
}

fn consume_drink(drink_sprite_query: Query<Entity, With<DrinkSprite>>, mut commands: Commands) {
    for entity in drink_sprite_query.iter() {
        commands.entity(entity).despawn();
    }
}

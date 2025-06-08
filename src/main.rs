use bevy::prelude::*;

use crate::engine::{GameState, game_runner::GameRunnerPlugin, asset_loader::{AudioAssets, ImageAssets}};
use crate::customers::dialogue::NextDialogueNode;
use bevy_asset_loader::prelude::*;

pub mod animation;
pub mod bar;
pub mod constants;
pub mod customers;
pub mod dialogue;
pub mod engine;
pub mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(create_window_plugin()),
        GameRunnerPlugin,
    ))
    .init_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .load_collection::<AudioAssets>()
            .load_collection::<ImageAssets>()
            .continue_to_state(GameState::Crafting),
    )
    .add_systems(OnEnter(GameState::Dialogues), set_zara_dialogue_start_node)
    .run();
}

fn set_zara_dialogue_start_node(mut commands: Commands) {
    commands.insert_resource(NextDialogueNode("ZaraDialogue".to_string()));
}

fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Cosmos on the Rocks".to_string(),
            ..default()
        }),
        ..default()
    }
}

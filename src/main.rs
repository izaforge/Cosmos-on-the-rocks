use bevy::prelude::*;

use crate::engine::{
    GameState,
    game_runner::GameRunnerPlugin,
};

pub mod animation;
pub mod bar;
pub mod constants;
pub mod customers;
pub mod dialogue;
pub mod engine;
pub mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(create_window_plugin()), GameRunnerPlugin))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .run();
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

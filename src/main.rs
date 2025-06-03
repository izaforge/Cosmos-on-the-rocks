use bevy::prelude::*;

use crate::engine::{GameState, game_runner::GameRunnerPlugin};

pub mod bar;
pub mod constants;
pub mod customers;
pub mod engine;
pub mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, GameRunnerPlugin))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .run();
}
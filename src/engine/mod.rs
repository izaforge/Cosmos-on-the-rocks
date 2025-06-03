use bevy::prelude::*;

pub mod asset_loader;
pub mod game_runner;
pub mod audio_controller;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Settings,
    CustomerInteraction,
    Crafting,
    EndNight,
}

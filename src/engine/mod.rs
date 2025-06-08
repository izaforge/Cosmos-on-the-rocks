use bevy::prelude::*;

pub mod asset_loader;
pub mod audio_controller;
pub mod game_runner;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Settings,
    Dialogues,
    Crafting,
    EndNight,
}

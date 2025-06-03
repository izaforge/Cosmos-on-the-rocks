use bevy::prelude::*;

pub mod asset_loader;
pub mod game_runner;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    AssetLoading,
    Settings,
    CustomerInteraction,
    Crafting,
    EndNight,
}

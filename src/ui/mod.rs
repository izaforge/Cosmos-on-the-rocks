use crate::{
    engine::GameState,
    ui::main_menu::{button_interaction_system, cleanup_menu, setup_main_menu},
    ui::crafting_menu::{cleanup_crafting_menu, setup_crafting_menu},
};
use bevy::prelude::*;

pub mod main_menu;
pub mod crafting_menu;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnEnter(GameState::Crafting), setup_crafting_menu)
            .add_systems(OnExit(GameState::Crafting), cleanup_crafting_menu);
    }
}

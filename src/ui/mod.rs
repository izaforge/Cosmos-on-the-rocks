use crate::{
    engine::GameState,
    ui::{
        crafting_ui::{crafting_button_interaction_system, setup_crafting_menu},
        main_menu::{button_interaction_system, cleanup_menu, setup_main_menu},
    },
};
use bevy::prelude::*;

pub mod crafting_ui;
pub mod emotion_ui;
pub mod ingredient_tooltip;
pub mod main_menu;
pub mod mood_ui;

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
            .add_systems(
                Update,
                (crafting_button_interaction_system,).run_if(in_state(GameState::Crafting)),
            );
    }
}

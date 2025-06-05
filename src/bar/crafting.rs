use bevy::prelude::*;

use crate::{bar::bar_counter::spawn_ingredients, engine::GameState};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CustomerInteraction), spawn_ingredients);
    }
}

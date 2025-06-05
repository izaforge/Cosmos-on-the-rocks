use bevy::prelude::*;

use crate::{
    animation::{sprite_animation::animate_spite, AnimationEvent}, bar::bar_counter::spawn_ingredients,
    engine::GameState,
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CustomerInteraction), spawn_ingredients)
            .add_systems(
                Update,
                animate_spite.run_if(in_state(GameState::CustomerInteraction)),
            )
            .add_event::<AnimationEvent>();
    }
}

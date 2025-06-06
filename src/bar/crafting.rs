use bevy::prelude::*;

use crate::{
    animation::{AnimationEvent, sprite_animation::animate_spite},
    bar::{glass::spawn_glass, ingredient::spawn_ingredients},
    engine::GameState,
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::CustomerInteraction),
            (spawn_ingredients, spawn_glass),
        )
        .add_systems(
            Update,
            animate_spite.run_if(in_state(GameState::CustomerInteraction)),
        )
        .add_plugins(SpritePickingPlugin)
        .add_event::<AnimationEvent>();
    }
}

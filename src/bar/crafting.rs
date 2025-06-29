use bevy::prelude::*;

use crate::{
    animation::{AnimationEvent, sprite_animation::animate_spite},
    bar::{
        bar_counter::{spawn_bartender, spawn_crafting_area},
        glass::spawn_glass,
    },
    engine::{GameState, audio_controller::play_crafting_bg},
    ingredients::spawn_ingredients,
    ui::{crafting_ui::setup_glass_ui, ingredient_tooltip::despawn_glass_full_indicator},
};

#[derive(Component)]
pub struct OnCraftingScreen;

#[derive(Component)]
pub enum CraftButtons {
    Craft,
    Reset,
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Crafting),
            (
                spawn_ingredients,
                (spawn_glass, setup_glass_ui).chain(),
                spawn_bartender,
                play_crafting_bg,
                spawn_crafting_area,
            ),
        )
        .add_systems(
            Update,
            (animate_spite, despawn_glass_full_indicator).run_if(in_state(GameState::Crafting)),
        )
        .add_systems(OnExit(GameState::Crafting), cleanup_crafting)
        .add_event::<AnimationEvent>();
    }
}

pub fn cleanup_crafting(mut commands: Commands, query: Query<Entity, With<OnCraftingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

use bevy::prelude::*;

use crate::{
    animation::{AnimationEvent, sprite_animation::animate_spite},
    customers::customer_sprites::get_character_sprites,
    dialogues::{DialogPlugin, DialogueState},
    engine::{GameState, asset_loader::ImageAssets, audio_controller::play_customer_bg},
    ingredients::{IngredientTaste, PrimaryEffect},
};

pub mod customer_sprites;

#[derive(Component)]
pub struct OnCustomerScreen;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DialogPlugin)
            .add_systems(
                OnEnter(GameState::Dialogues),
                (play_customer_bg, spawn_customer, spawn_bg),
            )
            .add_systems(
                Update,
                (animate_spite).run_if(in_state(GameState::Dialogues)),
            )
            .add_systems(OnExit(GameState::Dialogues), cleanup_customer)
            .add_event::<AnimationEvent>();
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Customer {
    pub name: String,
    pub preferred_taste: IngredientTaste,
    pub disliked_taste: IngredientTaste,
    pub satisfaction_score: f32,
    pub current_drink: Option<Entity>,
    pub dialogue_node: Option<String>,
    pub base_personality: Personality,
}

/// Base personalities for each patron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Personality {
    Secretive,  // Mystery
    Volatile,   // Zara
    Artificial, // Unit 734
    Creative,   // Coda
}

pub fn spawn_customer(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    dialogue_state: Res<State<DialogueState>>,
) {
    let characters: Vec<(Customer, Sprite, Transform)> =
        get_character_sprites(dialogue_state, image_assets);

    for (customer, sprite, transform) in characters {
        commands.spawn((OnCustomerScreen, customer, sprite, transform));
    }
}

fn spawn_bg(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite {
            image: image_assets.background_image.clone(),
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

pub fn cleanup_customer(
    mut commands: Commands,
    customer_query: Query<Entity, With<OnCustomerScreen>>,
) {
    for entity in customer_query.iter() {
        commands.entity(entity).despawn();
    }
}

use bevy::prelude::*;

use crate::{
    animation::{
        AnimationEvent,
        sprite_animation::{SpriteAnimState, animate_spite},
    },
    bar::ingredient::{IngredientTaste, PrimaryEffect},
    customers::dialogue::DialogPlugin,
    engine::{GameState, asset_loader::ImageAssets, audio_controller::play_customer_bg},
};

pub mod dialogue;

#[derive(Component)]
pub struct OnCustomerScreen;

#[derive(Component)]
pub struct DialogueAlignedCharacter;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DialogPlugin)
            .add_systems(
                OnEnter(GameState::Dialogues),
                (play_customer_bg, spawn_customer),
            )
            .add_systems(Update, (animate_spite, position_dialogue_aligned_characters).run_if(in_state(GameState::Dialogues)))
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
    pub preferred_effect: PrimaryEffect,
    pub disliked_effect: PrimaryEffect,
    pub satisfaction_score: f32,
    pub has_been_served: bool,
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

// New emotion components for customers
#[derive(Component)]
pub struct CustomerHappiness {
    pub value: u8, // 0-100 value
}

#[derive(Component)]
pub struct CustomerSadness {
    pub value: u8, // 0-100 value
}

#[derive(Component)]
pub struct CustomerAnger {
    pub value: u8, // 0-100 value
}

pub fn spawn_bartender(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(768, 1024);
    let customer_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        2,
        1,
        None,
        None,
    ));
    commands.spawn((
        Customer {
            name: "Bartender".to_string(),
            preferred_taste: IngredientTaste::Spicy,
            disliked_taste: IngredientTaste::Sweet,
            preferred_effect: PrimaryEffect::Healing,
            disliked_effect: PrimaryEffect::TruthInducing,
            satisfaction_score: 100.0,
            has_been_served: false,
            current_drink: None,
            dialogue_node: None,
            base_personality: Personality::Artificial,
        },
        CustomerHappiness { value: 50 },
        CustomerSadness { value: 20 },
        CustomerAnger { value: 30 },
        Sprite {
            image: image_assets.bartender.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: customer_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(192., 256.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(400., 0., 1.)),
        SpriteAnimState {
            start_index: 0,
            end_index: 1,
            timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
        OnCustomerScreen,
    ));
}

pub fn spawn_customer(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    // Spawn Zara as a customer with static image
    commands.spawn((
        Customer {
            name: "Zara".to_string(),
            preferred_taste: IngredientTaste::Umami,
            disliked_taste: IngredientTaste::Bitter,
            preferred_effect: PrimaryEffect::Energizing,
            disliked_effect: PrimaryEffect::MindEnhancing,
            satisfaction_score: 50.0,
            has_been_served: false,
            current_drink: None,
            dialogue_node: None,
            base_personality: Personality::Volatile,
        },
        CustomerHappiness { value: 50 },
        CustomerSadness { value: 20 },
        CustomerAnger { value: 30 },
        Sprite {
            image: image_assets.zara.clone(),
            custom_size: Some(Vec2::new(200., 300.)), // Sized to fit nicely in the scene
            ..default()
        },
        Transform::from_translation(Vec3::new(-400., 0., 1.)), // Initial position, will be adjusted by positioning system
        DialogueAlignedCharacter, // Mark this character for dialogue alignment
        OnCustomerScreen,
    ));
}

/// System to position dialogue-aligned characters relative to the dialogue box area
fn position_dialogue_aligned_characters(
    mut character_query: Query<&mut Transform, With<DialogueAlignedCharacter>>,
) {
    // Calculate the dialogue area position based on screen dimensions
    // The dialogue typically appears in the bottom 30% of the screen
    let screen_height = 1080.0; // From camera setup - fixed vertical viewport
    let dialogue_area_height_ratio = 0.3; // Dialogue takes up bottom 30% of screen
    let dialogue_top_y = -(screen_height * 0.5) + (screen_height * dialogue_area_height_ratio);
    
    // Position characters slightly above the dialogue area
    let character_y = dialogue_top_y + 50.0; // 50 pixels above dialogue area
    
    for mut transform in character_query.iter_mut() {
        // Keep X position the same, just adjust Y to align with dialogue
        transform.translation.y = character_y;
    }
}

// System to cleanup menu when exiting MainMenu state
pub fn cleanup_customer(mut commands: Commands, query: Query<Entity, With<OnCustomerScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

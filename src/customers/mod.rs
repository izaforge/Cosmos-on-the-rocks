use bevy::prelude::*;

use crate::{
    animation::{
        AnimationEvent,
        sprite_animation::{SpriteAnimState, animate_spite},
    },
    bar::ingredient::{IngredientTaste, PrimaryEffect},
    customers::dialogue::{DialogPlugin, DialogueState},
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
            .add_systems(Update, (animate_spite, position_dialogue_aligned_characters, handle_dialogue_state_changes).run_if(in_state(GameState::Dialogues)))
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
        Transform::from_translation(Vec3::new(400., -170., 1.)),
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
    dialogue_state: Res<DialogueState>,
) {
    // Spawn different customers based on dialogue state - matching the exact dialogue progression
    if !dialogue_state.bartender_monologue_played || !dialogue_state.bartender_drink_finished {
        // During bartender dialogues, no customer is visible
        return;
    } else if !dialogue_state.zara_dialogue_finished {
        // Spawn Zara during her first dialogue
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
    } else if dialogue_state.zara_dialogue_finished && !dialogue_state.coda_dialogue_finished {
        // Spawn Coda during his first dialogue (CodaDialogue)
        commands.spawn((
            Customer {
                name: "Coda".to_string(),
                preferred_taste: IngredientTaste::Sweet,
                disliked_taste: IngredientTaste::Sour,
                preferred_effect: PrimaryEffect::MindEnhancing,
                disliked_effect: PrimaryEffect::Energizing,
                satisfaction_score: 50.0,
                has_been_served: false,
                current_drink: None,
                dialogue_node: None,
                base_personality: Personality::Creative,
            },
            CustomerHappiness { value: 60 },
            CustomerSadness { value: 10 },
            CustomerAnger { value: 20 },
            Sprite {
                image: image_assets.coda.clone(), // Now using Coda's actual image
                custom_size: Some(Vec2::new(200., 300.)), // Sized to fit nicely in the scene
                ..default()
            },
            Transform::from_translation(Vec3::new(-400., 0., 1.)), // Initial position, will be adjusted by positioning system
            DialogueAlignedCharacter, // Mark this character for dialogue alignment
            OnCustomerScreen,
        ));
    } else if dialogue_state.coda_dialogue_finished && !dialogue_state.coda_second_visit {
        // Spawn Coda during his second visit (CodaSecondVisit)
        commands.spawn((
            Customer {
                name: "Coda".to_string(),
                preferred_taste: IngredientTaste::Sweet,
                disliked_taste: IngredientTaste::Sour,
                preferred_effect: PrimaryEffect::MindEnhancing,
                disliked_effect: PrimaryEffect::Energizing,
                satisfaction_score: 50.0,
                has_been_served: false,
                current_drink: None,
                dialogue_node: None,
                base_personality: Personality::Creative,
            },
            CustomerHappiness { value: 60 },
            CustomerSadness { value: 10 },
            CustomerAnger { value: 20 },
            Sprite {
                image: image_assets.coda.clone(), // Now using Coda's actual image
                custom_size: Some(Vec2::new(200., 300.)), // Sized to fit nicely in the scene
                ..default()
            },
            Transform::from_translation(Vec3::new(-400., 0., 1.)), // Initial position, will be adjusted by positioning system
            DialogueAlignedCharacter, // Mark this character for dialogue alignment
            OnCustomerScreen,
        ));
    } else {
        // Spawn Zara for her return dialogue (ZaraReturnDialogue) - this happens when all previous dialogues are finished
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
    let character_y = dialogue_top_y + 100.0; // 100 pixels above dialogue area
    
    for mut transform in character_query.iter_mut() {
        // Keep X position the same, just adjust Y to align with dialogue
        transform.translation.y = character_y;
    }
}

/// System to handle dialogue state changes and respawn appropriate customers
fn handle_dialogue_state_changes(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    dialogue_state: Res<DialogueState>,
    customer_query: Query<(Entity, &Customer)>,
    mut last_dialogue_state: Local<Option<DialogueState>>,
) {
    // Check if dialogue state has changed
    let current_state = DialogueState {
        bartender_monologue_played: dialogue_state.bartender_monologue_played,
        bartender_drink_finished: dialogue_state.bartender_drink_finished,
        zara_dialogue_finished: dialogue_state.zara_dialogue_finished,
        coda_dialogue_finished: dialogue_state.coda_dialogue_finished,
        coda_second_visit: dialogue_state.coda_second_visit,
    };
    
    if let Some(ref last_state) = *last_dialogue_state {
        // Compare states to see if anything changed
        if last_state.zara_dialogue_finished != current_state.zara_dialogue_finished ||
           last_state.coda_dialogue_finished != current_state.coda_dialogue_finished ||
           last_state.coda_second_visit != current_state.coda_second_visit {
            
            // Clean up existing customers (except bartender)
            for (entity, customer) in customer_query.iter() {
                if customer.name != "Bartender" {
                    commands.entity(entity).despawn();
                }
            }
            
            // Spawn the appropriate customer for the new state
            spawn_appropriate_customer(&mut commands, &image_assets, &dialogue_state);
        }
    }
    
    *last_dialogue_state = Some(current_state);
}

/// Helper function to spawn the appropriate customer based on dialogue state
fn spawn_appropriate_customer(
    commands: &mut Commands,
    image_assets: &Res<ImageAssets>,
    dialogue_state: &Res<DialogueState>,
) {
    if !dialogue_state.bartender_monologue_played || !dialogue_state.bartender_drink_finished {
        // During bartender dialogues, no customer is visible
        return;
    } else if !dialogue_state.zara_dialogue_finished {
        // Spawn Zara during her first dialogue
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
                custom_size: Some(Vec2::new(200., 300.)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-400., 0., 1.)),
            DialogueAlignedCharacter,
            OnCustomerScreen,
        ));
    } else if dialogue_state.zara_dialogue_finished && !dialogue_state.coda_dialogue_finished {
        // Spawn Coda during his first dialogue (CodaDialogue)
        commands.spawn((
            Customer {
                name: "Coda".to_string(),
                preferred_taste: IngredientTaste::Sweet,
                disliked_taste: IngredientTaste::Sour,
                preferred_effect: PrimaryEffect::MindEnhancing,
                disliked_effect: PrimaryEffect::Energizing,
                satisfaction_score: 50.0,
                has_been_served: false,
                current_drink: None,
                dialogue_node: None,
                base_personality: Personality::Creative,
            },
            CustomerHappiness { value: 60 },
            CustomerSadness { value: 10 },
            CustomerAnger { value: 20 },
            Sprite {
                image: image_assets.coda.clone(),
                custom_size: Some(Vec2::new(200., 300.)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-400., 0., 1.)),
            DialogueAlignedCharacter,
            OnCustomerScreen,
        ));
    } else if dialogue_state.coda_dialogue_finished && !dialogue_state.coda_second_visit {
        // Spawn Coda during his second visit (CodaSecondVisit)
        commands.spawn((
            Customer {
                name: "Coda".to_string(),
                preferred_taste: IngredientTaste::Sweet,
                disliked_taste: IngredientTaste::Sour,
                preferred_effect: PrimaryEffect::MindEnhancing,
                disliked_effect: PrimaryEffect::Energizing,
                satisfaction_score: 50.0,
                has_been_served: false,
                current_drink: None,
                dialogue_node: None,
                base_personality: Personality::Creative,
            },
            CustomerHappiness { value: 60 },
            CustomerSadness { value: 10 },
            CustomerAnger { value: 20 },
            Sprite {
                image: image_assets.coda.clone(),
                custom_size: Some(Vec2::new(200., 300.)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-400., 0., 1.)),
            DialogueAlignedCharacter,
            OnCustomerScreen,
        ));
    } else {
        // Spawn Zara for her return dialogue (ZaraReturnDialogue)
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
                custom_size: Some(Vec2::new(200., 300.)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-400., 0., 1.)),
            DialogueAlignedCharacter,
            OnCustomerScreen,
        ));
    }
}

// System to cleanup menu when exiting MainMenu state
pub fn cleanup_customer(mut commands: Commands, query: Query<Entity, With<OnCustomerScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

use bevy::prelude::*;

use crate::{
    animation::{
        AnimationEvent,
        sprite_animation::{SpriteAnimState, animate_spite},
    },
    dialogues::{DialogPlugin, DialogueState},
    engine::{GameState, asset_loader::ImageAssets, audio_controller::play_customer_bg},
    ingredients::{IngredientTaste, PrimaryEffect},
};

#[derive(Component)]
pub struct OnCustomerScreen;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DialogPlugin)
            .add_systems(
                OnEnter(GameState::Dialogues),
                (play_customer_bg, spawn_customer),
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
    match dialogue_state.get() {
        DialogueState::BartenderMonologue => {
            commands.spawn((
                OnCustomerScreen,
                Customer {
                    name: "Bartender".to_string(),
                    preferred_taste: IngredientTaste::Spicy,
                    disliked_taste: IngredientTaste::Sweet,
                    satisfaction_score: 100.0,
                    current_drink: None,
                    dialogue_node: None,
                    base_personality: Personality::Artificial,
                },
                Sprite {
                    image: image_assets.bartenter_full.clone(),
                    custom_size: Some(Vec2::new(192.0, 256.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(400., 0., 1.)),
            ));
        }
        DialogueState::CarlEnters => {}
        DialogueState::ZaraEnters => {
            commands.spawn((
                OnCustomerScreen,
                Customer {
                    name: "Carl".to_string(),
                    preferred_taste: IngredientTaste::Spicy,
                    disliked_taste: IngredientTaste::Sweet,
                    satisfaction_score: 100.0,
                    current_drink: None,
                    dialogue_node: None,
                    base_personality: Personality::Artificial,
                },
                Sprite {
                    image: image_assets.carl_full.clone(),
                    custom_size: Some(Vec2::new(192.0, 256.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(400., 0., 1.)),
            ));
        }
        DialogueState::CodaEnters => {}
        DialogueState::Mystery => todo!(),
    }
}

pub fn cleanup_customer(
    mut commands: Commands,
    customer_query: Query<Entity, With<OnCustomerScreen>>,
) {
    for entity in customer_query.iter() {
        commands.entity(entity).despawn();
    }
}

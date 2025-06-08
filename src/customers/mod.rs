use bevy::prelude::*;

use crate::{
    animation::{
        AnimationEvent,
        sprite_animation::{SpriteAnimState, animate_spite},
    },
    customers::dialogue::DialogPlugin,
    engine::{GameState, asset_loader::ImageAssets, audio_controller::play_customer_bg},
};

pub mod dialogue;

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
            .add_systems(Update, animate_spite.run_if(in_state(GameState::Dialogues)))
            .add_systems(OnExit(GameState::Dialogues), cleanup_customer)
            .add_event::<AnimationEvent>();
    }
}

#[derive(Component)]
#[require(Sprite, SpriteAnimState, Transform)]
pub struct Customer {
    pub name: String,
    pub faction: Factions,
    pub likes: Vec<String>,
    pub dislikes: Vec<String>,
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

pub enum Factions {
    Rebels,
    Empire,
    BountyHunter,
    Unknown,
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
            faction: Factions::BountyHunter,
            likes: vec!["Drinks".to_string(), "Money".to_string()],
            dislikes: vec!["Rude Customers".to_string()],
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
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Create a texture atlas for Zara with 4 animation frames
    let frame_size = UVec2::new(32, 48); // Adjust based on your sprite sheet dimensions
    let zara_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size, 4, // 4 frames horizontally
        1, // 1 row
        None, None,
    ));

    // Spawn Zara as a customer
    commands.spawn((
        Customer {
            name: "Zara".to_string(),
            faction: Factions::Empire,
            likes: vec!["Strong Drinks".to_string(), "Blue Proxima".to_string()],
            dislikes: vec!["Light Drinks".to_string(), "Small Talk".to_string()],
        },
        CustomerHappiness { value: 50 },
        CustomerSadness { value: 20 },
        CustomerAnger { value: 30 },
        Sprite {
            image: image_assets.zara.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: zara_layout_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::new(96., 144.)), // Scaled up for visibility
            ..default()
        },
        Transform::from_translation(Vec3::new(-400., 0., 1.)), // Position on the left side
        SpriteAnimState {
            start_index: 0,
            end_index: 3,                                           // 4 frames (0-3)
            timer: Timer::from_seconds(0.25, TimerMode::Repeating), // 4 frames per second
        },
        OnCustomerScreen,
    ));
}

// System to cleanup menu when exiting MainMenu state
pub fn cleanup_customer(mut commands: Commands, query: Query<Entity, With<OnCustomerScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

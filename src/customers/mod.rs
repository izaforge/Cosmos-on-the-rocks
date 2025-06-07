use bevy::prelude::*;

use crate::{
    animation::sprite_animation::SpriteAnimState,
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
            .add_systems(OnEnter(GameState::CustomerInteraction), play_customer_bg)
            .add_systems(OnExit(GameState::CustomerInteraction), cleanup_customer);
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

// System to cleanup menu when exiting MainMenu state
pub fn cleanup_customer(mut commands: Commands, query: Query<Entity, With<OnCustomerScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

use bevy::prelude::*;

use crate::{
    animation::sprite_animation::SpriteAnimState, bar::crafting::OnCraftingScreen,
    engine::asset_loader::ImageAssets,
};

pub fn spawn_crafting_area(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite {
            image: image_assets.bar_shelf.clone(),
            custom_size: Some(Vec2::new(1536.0, 1024.0)),
            ..default()
        },
        Transform::from_xyz(-190.0, 50.0, -9.0),
        OnCraftingScreen,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.bar_counter.clone(),
            custom_size: Some(Vec2::new(1536.0, 1024.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, -400.0, -8.0),
        OnCraftingScreen,
    ));
}

pub fn spawn_bartender(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let frame_size = UVec2::new(768, 1024);
    let bartender_layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        frame_size as UVec2,
        2,
        1,
        None,
        None,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.bartender.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: bartender_layout_handle,
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
        OnCraftingScreen,
    ));
}

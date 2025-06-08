
use bevy::prelude::*;

use crate::{bar::crafting::OnCraftingScreen, engine::asset_loader::ImageAssets};

pub fn spawn_crafting_area(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite {
            image: image_assets.background_image.clone(),
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
        OnCraftingScreen,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.bar_shelf.clone(),
            custom_size: Some(Vec2::new(1536.0, 1024.0)),
            ..default()
        },
        Transform::from_xyz(-80.0, 50.0, -9.0),
        OnCraftingScreen,
    ));
    commands.spawn((
        Sprite {
            image: image_assets.bar_counter.clone(),
            custom_size: Some(Vec2::new(1024.0, 1024.0)),
            ..default()
        },
        Transform::from_xyz(-180.0, -200.0, -8.0),
        OnCraftingScreen,
    ));
}
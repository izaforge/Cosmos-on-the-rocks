use std::collections::HashMap;

use bevy::prelude::*;
use uuid::Uuid;

use crate::engine::asset_loader::ImageAssets;

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform)]
pub struct Glass {
    pub capacity: f32,
    pub shape: GlassShape,
    pub ingredients: HashMap<Uuid, f32>,
}

#[derive(Clone, Debug)]
pub enum GlassShape {
    Whiskey,
    Wine,
    Jar,
}

pub fn spawn_glass(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let glass_sprite = Sprite {
        image: image_assets.wine_glass.clone(),
        custom_size: Some(Vec2::new(128., 128.)),
        ..default()
    };
    let crafting_glass = Glass {
        capacity: 100.0,
        shape: GlassShape::Wine,
        ingredients: HashMap::new(),
    };
    commands.spawn((
        crafting_glass,
        glass_sprite,
        Transform::from_translation(Vec3::new(200., 0., 1.)),
    ));
}

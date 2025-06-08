use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    bar::{
        crafting::OnCraftingScreen,
        drinks::Drink,
        ingredient::{Ingredient, IngredientTaste},
    },
    engine::{GameState, asset_loader::ImageAssets},
};

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform)]
pub struct Glass {
    pub capacity: f32,
    pub shape: GlassShape,
    pub ingredients: HashMap<Entity, f32>,
    pub taste: HashMap<IngredientTaste, f32>,
}

impl Glass {
    pub fn get_current_volume(&self) -> f32 {
        self.ingredients.values().sum()
    }
    pub fn reset(&mut self) {
        self.ingredients.clear();
        self.taste.clear();
    }
}

#[derive(Clone, Debug)]
pub enum GlassShape {
    Whiskey,
    Wine,
    Cocktail,
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
        taste: HashMap::new(),
    };
    commands
        .spawn((
            crafting_glass,
            glass_sprite,
            Transform::from_translation(Vec3::new(200., 0., 1.)),
            OnCraftingScreen,
            Pickable::default(),
        ))
        .observe(
            |_: Trigger<Pointer<Click>>,
             mut glass_query: Query<(&mut Glass, &mut Sprite)>,
             image_assets: Res<ImageAssets>| {
                for (mut glass, mut sprite) in glass_query.iter_mut() {
                    glass.shape = GlassShape::Whiskey;
                    sprite.image = image_assets.whiskey_glass.clone();
                }
            },
        );
}

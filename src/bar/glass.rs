use std::collections::HashMap;

use bevy::{
    picking::{
        events::{Click, Out, Over, Pointer},
        prelude::Pickable,
    },
    prelude::*,
};

use crate::{
    bar::{
        crafting::OnCraftingScreen,
        drinks::Drink,
        ingredient::{Ingredient, IngredientTaste, PrimaryEffect, SecondaryEffect},
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
    pub effect: HashMap<PrimaryEffect, f32>,
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
        effect: HashMap::new(),
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
            |event: Trigger<Pointer<Click>>,
             mut query: Query<(&mut Glass, &mut Sprite)>,
             image_assets: Res<ImageAssets>| {
                if let Ok((mut glass, mut sprite)) = query.get_mut(event.target) {
                    let (next_shape, new_image) = match glass.shape {
                        GlassShape::Wine => {
                            (GlassShape::Whiskey, image_assets.whiskey_glass.clone())
                        }
                        GlassShape::Whiskey => {
                            (GlassShape::Cocktail, image_assets.cocktail_glass.clone())
                        }
                        GlassShape::Cocktail => (GlassShape::Wine, image_assets.wine_glass.clone()),
                    };
                    info!(
                        "Switched glass shape from {:?} to {:?}",
                        glass.shape, next_shape
                    );
                    glass.shape = next_shape;
                    sprite.image = new_image;
                }
            },
        )
        .observe(|ev: Trigger<Pointer<Over>>, glass_query: Query<&Glass>| {
            if let Ok(glass) = glass_query.get(ev.target()) {
                let volume = glass.get_current_volume();
                info!(
                    "Hovering over glass - Current volume: {:.1}/{:.1}",
                    volume, glass.capacity
                );
            }
        })
        .observe(|_: Trigger<Pointer<Out>>| {
            // Hover end - tooltip will be handled by the UI system
        });
}

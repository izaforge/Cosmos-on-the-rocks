use bevy::{picking::prelude::Pickable, prelude::*};

use crate::{
    animation::sprite_animation::SpriteAnimState,
    bar::crafting::OnCraftingScreen,
    engine::asset_loader::ImageAssets,
    ingredients::{animated_ingredients::get_ice_gels, static_ingredients::get_static_ingredients},
    ui::ingredient_tooltip::{ingredient_hover, ingredient_hover_out, ingredient_pressed},
};

pub mod animated_ingredients;
pub mod static_ingredients;

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform, SpriteAnimState)]
pub struct Ingredient {
    pub name: String,
    pub description: String,
    pub ingredient_profile: IngredientProfile,
}

#[derive(Clone, Debug)]
pub struct IngredientProfile {
    pub size: f32,
    pub taste: IngredientTaste,
    pub primary_effect: PrimaryEffect,
    pub secondary_effect: SecondaryEffect,
    pub hazard: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum IngredientTaste {
    None,
    Sweet,
    Sour,
    Bitter,
    Citrus,
    Umami,
    Spicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimaryEffect {
    Calming,
    Energizing,
    MindEnhancing,
    CourageBoosting,
    TruthInducing,
    Healing,
}

#[derive(Clone, Debug)]
pub enum SecondaryEffect {
    Euphoric(EffectCondition),
    Agitated(EffectCondition),
    Hallucinogenic(EffectCondition),
    Paranoia(EffectCondition),
    Aggresive(EffectCondition),
    Sedated(EffectCondition),
}

#[derive(Clone, Debug)]
pub struct EffectCondition {
    pub volume_needed: f32,
    pub catalyst: Option<Entity>,
}

pub fn spawn_ingredients(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let animated_ingredients = get_ice_gels(&image_assets, texture_atlases);
    let static_ingredients = get_static_ingredients(&image_assets);
    for (ingredient, sprite, transform, anim_state) in animated_ingredients {
        commands
            .spawn((
                ingredient,
                sprite,
                transform,
                anim_state,
                Pickable::default(),
                OnCraftingScreen,
            ))
            .observe(ingredient_pressed)
            .observe(ingredient_hover)
            .observe(ingredient_hover_out);
    }
    for (ingredient, sprite, transform) in static_ingredients {
        commands
            .spawn((
                ingredient,
                sprite,
                transform,
                Pickable::default(),
                OnCraftingScreen,
            ))
            .observe(ingredient_pressed)
            .observe(ingredient_hover)
            .observe(ingredient_hover_out);
    }
}

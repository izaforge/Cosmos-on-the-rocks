use std::collections::HashMap;

use bevy::{
    prelude::*,
    picking::{
        events::{Pointer, Click, Over, Out},
        prelude::Pickable,
    },
};

use crate::{
    bar::{crafting::OnCraftingScreen, drinks::Drink, ingredient::{Ingredient, IngredientTaste}},
    engine::{GameState, asset_loader::ImageAssets},
    customers::dialogue::NextDialogueNode,
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
             mut commands: Commands,
             mut glass_query: Query<&mut Glass>,
             mut game_state: ResMut<NextState<GameState>>| {
                for glass in glass_query.iter_mut() {
                    let drink = Drink::from(glass.clone());
                    info!("Crafted {:#?}", drink);
                    
                    // Spawn the drink entity - this will trigger convert_drink_to_effects
                    commands.spawn(drink);
                    
                    // Set the next dialogue node to continue after drink completion
                    commands.insert_resource(NextDialogueNode("ZaraAfterDrink".to_string()));
                    
                    game_state.set(GameState::CustomerInteraction);
                }
            },
        )
        .observe(
            |ev: Trigger<Pointer<Over>>, glass_query: Query<&Glass>| {
                if let Ok(glass) = glass_query.get(ev.target()) {
                    let volume = glass.get_current_volume();
                    info!("Hovering over glass - Current volume: {:.1}/{:.1}", volume, glass.capacity);
                }
            },
        )
        .observe(
            |_: Trigger<Pointer<Out>>| {
                // Hover end - tooltip will be handled by the UI system
            },
        );
}

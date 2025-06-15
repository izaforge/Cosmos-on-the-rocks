use bevy::prelude::*;

use crate::{
    customers::{Customer, Personality},
    dialogues::DialogueState,
    engine::asset_loader::ImageAssets,
    ingredients::IngredientTaste,
};

pub fn get_character_sprites(
    dialogue_state: Res<State<DialogueState>>,
    image_assets: Res<ImageAssets>,
) -> Vec<(Customer, Sprite, Transform)> {
    let mut chars = Vec::new();
    let bartender = Customer {
        name: "Bartender".to_string(),
        preferred_taste: IngredientTaste::Spicy,
        disliked_taste: IngredientTaste::Sweet,
        satisfaction_score: 100.0,
        current_drink: None,
        dialogue_node: None,
        base_personality: Personality::Artificial,
    };
    let bartender_sprite = Sprite {
        image: image_assets.bartenter_full.clone(),
        custom_size: Some(Vec2::new(192.0, 256.0)),
        ..default()
    };
    let bartender_transform = Transform::from_translation(Vec3::new(400., 0., 1.));
    let customer_transform = Transform::from_translation(Vec3::new(-400., 0., 1.));

    chars.push((bartender, bartender_sprite, bartender_transform));
    match dialogue_state.get() {
        DialogueState::BartenderMonologue => {}
        DialogueState::CarlEnters => {
            let (carl, carl_sprite) = get_carl(image_assets);
            chars.push((carl, carl_sprite, customer_transform));
        }
        DialogueState::ZaraEnters => {
            let (carl, carl_sprite) = get_zara(image_assets);
            chars.push((carl, carl_sprite, customer_transform));
        }
        DialogueState::CodaEnters => {
            let (carl, carl_sprite) = get_coda(image_assets);
            chars.push((carl, carl_sprite, customer_transform));
        }
        DialogueState::MysteryEnters => todo!(),
    }
    chars
}

fn get_carl(image_assets: Res<ImageAssets>) -> (Customer, Sprite) {
    let carl = Customer {
        name: "Carl".to_string(),
        preferred_taste: IngredientTaste::Spicy,
        disliked_taste: IngredientTaste::Umami,
        satisfaction_score: 100.0,
        current_drink: None,
        dialogue_node: None,
        base_personality: Personality::Creative,
    };
    let carl_sprite = Sprite {
        image: image_assets.carl_full.clone(),
        custom_size: Some(Vec2::new(192.0, 256.0)),
        ..default()
    };
    (carl, carl_sprite)
}

fn get_zara(image_assets: Res<ImageAssets>) -> (Customer, Sprite) {
    let zara = Customer {
        name: "Zara".to_string(),
        preferred_taste: IngredientTaste::Bitter,
        disliked_taste: IngredientTaste::Spicy,
        satisfaction_score: 100.0,
        current_drink: None,
        dialogue_node: None,
        base_personality: Personality::Volatile,
    };
    let zara_sprite = Sprite {
        image: image_assets.zara.clone(),
        custom_size: Some(Vec2::new(192.0, 256.0)),
        ..default()
    };
    (zara, zara_sprite)
}

fn get_coda(image_assets: Res<ImageAssets>) -> (Customer, Sprite) {
    let coda = Customer {
        name: "Coda".to_string(),
        preferred_taste: IngredientTaste::Sweet,
        disliked_taste: IngredientTaste::Umami,
        satisfaction_score: 100.0,
        current_drink: None,
        dialogue_node: None,
        base_personality: Personality::Creative,
    };
    let coda_sprite = Sprite {
        image: image_assets.coda.clone(),
        custom_size: Some(Vec2::new(192.0, 256.0)),
        ..default()
    };
    (coda, coda_sprite)
}

use bevy::prelude::*;

#[derive(Component)]
pub struct Drink {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<Entity>,
    pub glass: Entity,
    pub effect: Option<DrinkEffect>,
}

pub enum DrinkEffect {
    Calm,
    Energize,
    EnhanceMind,
}

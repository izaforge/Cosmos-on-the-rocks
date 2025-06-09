use bevy::prelude::*;

pub struct ShowIngredientTooltip {
    pub ingredient_name: String,
}

#[derive(Component)]
pub struct IngredientTooltip;

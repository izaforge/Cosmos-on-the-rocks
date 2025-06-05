use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
#[require(Sprite, Transform)]
pub struct Glass {
    pub capacity: f32,
    pub shape: GlassShape,
}

#[derive(Clone, Debug)]
pub enum GlassShape {
    Whiskey,
    Wine,
    Jar,
}

use bevy::prelude::*;

pub mod sprite_animation;
pub mod sprite_picking;

#[derive(Event)]
pub struct AnimationEvent {
    pub entity: Entity,
    pub kind: AnimationEventKind,
}

pub enum AnimationEventKind {
    Finished,
    Interaction,
}

use bevy::prelude::*;

#[derive(Event)]
pub struct AnimationEvent {
    pub entity: Entity,
    pub kind: AnimationEventKind,
}

pub enum AnimationEventKind {
    Finished,
    Interaction,
}

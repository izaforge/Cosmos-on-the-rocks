#[derive(Component)]
pub struct SpriteAnimState {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
}

pub fn animate_spite(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut SpriteAnimState)>,
    mut event_writer_anim: EventWriter<AnimationEvent>,
) {
    for (entity, mut sprite, mut anim_state) in query.iter_mut() {
        anim_state.timer.tick(time.delta());
        if anim_state.timer.finished() {
            if let Some(texture_atlas) = &mut sprite.texture_atlas {
                texture_atlas.index += 1;
                if texture_atlas.index > anim_state.end_index {
                    texture_atlas.index = anim_state.start_index;
                    event_writer_anim.write(AnimationEvent {
                        kind: AnimationEventKind::Finished,
                        entity,
                    });
                }
            }
        }
    }
}
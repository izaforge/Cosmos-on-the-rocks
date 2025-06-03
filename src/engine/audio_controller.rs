use bevy::prelude::*;
use bevy_seedling::prelude::*;

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub fn play_sound(mut commands: Commands, server: Res<AssetServer>) {
    // Play a sound!
    commands.spawn(SamplePlayer::new(server.load("assets/audio/HoliznaCC0 - Space!.mp3")));

    // Play a sound... with effects :O
    commands.spawn((
        SamplePlayer::new(server.load("audio/Ketsa - Drifting Space Jazz.mp3")).looping(),
        sample_effects![LowPassNode { frequency: 500.0 }],
    ));
}
use bevy::prelude::*;
use bevy_seedling::prelude::*;

use crate::engine::asset_loader::AudioAssets;

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

pub fn play_bg_sound(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    // Play a sound!
    commands.spawn(SamplePlayer::new(audio_assets.background.clone()));

    // Play a sound... with effects :O
    commands.spawn((
        SamplePlayer::new(audio_assets.background2.clone()).looping(),
        sample_effects![LowPassNode { frequency: 500.0 }],
    ));
}
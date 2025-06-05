use bevy::prelude::*;
use bevy_seedling::prelude::*;

use crate::{customers::OnCustomerScreen, engine::asset_loader::AudioAssets};

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, _app: &mut App) {
        todo!()
    }
}

pub fn play_customer_bg_sound(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    // Play a sound!
    //commands.spawn(SamplePlayer::new(audio_assets.background.clone()));

    // Play a sound... with effects :O
    commands.spawn((
        SamplePlayer::new(audio_assets.background2.clone()).looping(),
        sample_effects![LowPassNode { frequency: 500.0 }],
        OnCustomerScreen,
    ));
}

use bevy::prelude::*;
use bevy_seedling::prelude::*;

use crate::{
    bar::crafting::OnCraftingScreen, customers::OnCustomerScreen, engine::asset_loader::AudioAssets,
};

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, _app: &mut App) {
        todo!()
    }
}

pub fn play_customer_bg(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        SamplePlayer::new(audio_assets.background.clone()).looping(),
        sample_effects![LowPassNode { frequency: 500.0 }],
        OnCustomerScreen,
    ));
}

pub fn play_crafting_bg(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        SamplePlayer::new(audio_assets.background2.clone()).looping(),
        OnCraftingScreen,
    ));
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "characters/bartender.png")]
    pub bartender: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Ketsa - Drifting Space Jazz.mp3")]
    pub background: Handle<AudioSource>,
    #[asset(path = "audio/HoliznaCC0 - Space!.mp3")]
    pub background2: Handle<AudioSource>
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "characters/bartender.png")]
    pub bartender: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Ketsa - Drifting Space Jazz.ogg")]
    pub background: Handle<Sample>,
    #[asset(path = "audio/HoliznaCC0 - Space!.ogg")]
    pub background2: Handle<Sample>,
}

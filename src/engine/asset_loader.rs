use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/characters/bartender.png")]
    pub bartender: Handle<Image>,
    #[asset(path = "images/bar/wine_glass.png")]
    pub wine_glass: Handle<Image>,
    #[asset(path = "images/bar/ingredients/blue_icegel.png")]
    pub blue_icegel: Handle<Image>,
    #[asset(path = "images/bar/ingredients/red_icegel.png")]
    pub red_icegel: Handle<Image>,
    #[asset(path = "images/bar/ingredients/green_icegel.png")]
    pub green_icegel: Handle<Image>,
    #[asset(path = "images/bar/ingredients/fizzion_mist.png")]
    pub fizzion_mist: Handle<Image>,
    #[asset(path = "images/bar/ingredients/synth_vapor.png")]
    pub synth_vapor: Handle<Image>,
    #[asset(path = "images/bar/ingredients/circuit_juice.png")]
    pub circuit_juice: Handle<Image>,
    #[asset(path = "images/bar/ingredients/void_reserve.png")]
    pub void_reserve: Handle<Image>,
    #[asset(path = "images/bar/ingredients/sweetflux.png")]
    pub sweetflux: Handle<Image>,
    #[asset(path = "images/bar/ingredients/citraplasm.png")]
    pub citraplasm: Handle<Image>,
    #[asset(path = "images/bar/bg.png")]
    pub background_image: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Ketsa - Drifting Space Jazz.ogg")]
    pub background: Handle<Sample>,
    #[asset(path = "audio/HoliznaCC0 - Space!.ogg")]
    pub background2: Handle<Sample>,
}

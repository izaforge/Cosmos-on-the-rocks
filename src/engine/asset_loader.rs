use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/characters/bartender.png")]
    pub bartender: Handle<Image>,
    #[asset(path = "images/characters/bartenter_full.png")]
    pub bartenter_full: Handle<Image>,
    #[asset(path = "images/characters/carl_full.png")]
    pub carl_full: Handle<Image>,
    #[asset(path = "images/characters/zara.png")]
    pub zara: Handle<Image>,
    #[asset(path = "images/characters/coda.png")]
    pub coda: Handle<Image>,
    #[asset(path = "images/bar/glasses/whiskey_glass.png")]
    pub whiskey_glass: Handle<Image>,
    #[asset(path = "images/bar/glasses/wine_glass.png")]
    pub wine_glass: Handle<Image>,
    #[asset(path = "images/bar/glasses/cocktail_glass.png")]
    pub cocktail_glass: Handle<Image>,
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
    #[asset(path = "images/bar/bar_counter.png")]
    pub bar_counter: Handle<Image>,
    #[asset(path = "images/bar/bar_shelf.png")]
    pub bar_shelf: Handle<Image>,
    #[asset(path = "images/bar/drinks/binary_barrel.png")]
    pub binary_barrel: Handle<Image>,
    #[asset(path = "images/bar/drinks/botanica_surge.png")]
    pub botanica_surge: Handle<Image>,
    #[asset(path = "images/bar/drinks/cosmopolitan.png")]
    pub cosmopolitan: Handle<Image>,
    #[asset(path = "images/bar/drinks/echo_bloom.png")]
    pub echo_bloom: Handle<Image>,
    #[asset(path = "images/bar/drinks/event_horizon.png")]
    pub event_horizon: Handle<Image>,
    #[asset(path = "images/bar/drinks/old_memory.png")]
    pub old_memory: Handle<Image>,
    #[asset(path = "images/bar/drinks/synth_cascade.png")]
    pub synth_cascade: Handle<Image>,
    #[asset(path = "images/bar/drinks/cryo_drop.png")]
    pub cryo_drop: Handle<Image>,
    #[asset(path = "images/bar/drinks/stellar_lumen.png")]
    pub stellar_lumen: Handle<Image>,
    #[asset(path = "images/bar/drinks/zero_phase.png")]
    pub zero_phase: Handle<Image>,
    #[asset(path = "images/ui/pop.png")]
    pub pop: Handle<Image>,
    #[asset(path = "images/dialogue/talk_bg.png")]
    pub talk_background: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Ketsa - Drifting Space Jazz.ogg")]
    pub background: Handle<Sample>,
    #[asset(path = "audio/HoliznaCC0 - Space!.ogg")]
    pub background2: Handle<Sample>,
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::prelude::*;

use crate::{
    engine::{GameState, asset_loader::ImageAssets},
    ui::GameUiPlugin,
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SeedlingPlugin::default(),
            GameUiPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::CustomerInteraction),
        )
        .add_systems(Startup, (setup_camera, play_sound))
        ;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn play_sound(mut commands: Commands, server: Res<AssetServer>) {
    // Play a sound!
    commands.spawn(SamplePlayer::new(server.load("assets\\audio\\HoliznaCC0 - Space!.mp3")));

    // Play a sound... with effects :O
    commands.spawn((
        SamplePlayer::new(server.load("audio/Ketsa - Drifting Space Jazz.mp3")).looping(),
        sample_effects![LowPassNode { frequency: 500.0 }],
    ));
}

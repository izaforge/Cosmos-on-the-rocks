use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::prelude::*;

use crate::{
    bar::crafting::CraftingPlugin,
    customers::CustomerPlugin,
    engine::{
        GameState,
        asset_loader::{AudioAssets, ImageAssets},
    },
    ui::GameUiPlugin,
};

pub struct GameRunnerPlugin;

impl Plugin for GameRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SeedlingPlugin::default(),
            GameUiPlugin,
            CustomerPlugin,
            CraftingPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::CustomerInteraction),
        )
        .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

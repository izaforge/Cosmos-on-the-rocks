use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

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
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<AudioAssets>()
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Dialogues),
        )
        .add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
pub struct MainGameCamera;

fn setup_camera(mut commands: Commands) {
    let main_camera = Camera2d::default();
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 1080.0,
        },
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((main_camera, MainGameCamera, projection));
}

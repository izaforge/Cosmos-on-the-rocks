use bevy::prelude::*;

use cosmos_on_the_rocks::engine::{GameState, game_runner::GameRunnerPlugin, asset_loader::{AudioAssets, ImageAssets}};
use cosmos_on_the_rocks::customers::dialogue::NextDialogueNode;
use bevy_asset_loader::prelude::*;

// Asset structs
#[derive(Resource)]
struct GameAssets {
    background: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        // Add our game runner plugin which includes all other game systems
        GameRunnerPlugin,
    ))
    .init_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .load_collection::<AudioAssets>()
            .load_collection::<ImageAssets>()
            .continue_to_state(GameState::Crafting),
    )
    .add_systems(OnEnter(GameState::Dialogues), set_zara_dialogue_start_node)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        background: asset_server.load("dialogue/images/bg.png"),
    };
    commands.insert_resource(game_assets);
}

fn setup_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    _windows: Query<&Window>,
) {
    // Add background as a full-screen sprite with smooth scaling
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Visible,
        BackgroundImage,
        Sprite {
            image: game_assets.background.clone(),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
    ));
}

// Component for background to track it
#[derive(Component)]
struct BackgroundImage;

// System to ensure the background fills the screen
fn ensure_background_fills_screen(
    windows: Query<&Window>,
    mut background_query: Query<&mut Transform, With<BackgroundImage>>,
    images: Res<Assets<Image>>,
    game_assets: Res<GameAssets>,
) {
    if let Ok(window) = windows.single() {
        // Get image dimensions
        if let Some(image) = images.get(&game_assets.background) {
            let image_width = image.width() as f32;
            let image_height = image.height() as f32;
            
            // Calculate scale to fill screen while maintaining aspect ratio
            let scale_x = window.resolution.width() / image_width;
            let scale_y = window.resolution.height() / image_height;
            let scale = scale_x.max(scale_y); // Use max to ensure full coverage
            
            if let Ok(mut transform) = background_query.single_mut() {
                // Apply smooth scaling with interpolation
                transform.scale = Vec3::new(scale, scale, 1.0);
            }
        }
    }
}

fn set_zara_dialogue_start_node(mut commands: Commands) {
    commands.insert_resource(NextDialogueNode("ZaraDialogue".to_string()));
}
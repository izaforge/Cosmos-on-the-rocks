use bevy::prelude::*;

use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

// Asset structs
#[derive(Resource)]
struct GameAssets {
    background: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        // Register the Yarn Spinner plugin using its default settings, which will look for Yarn files in the "dialogue" folder.
        YarnSpinnerPlugin::new(),
        // Initialize the bundled example UI
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
        .add_systems(Startup, (setup_camera, load_assets))
        .add_systems(Update, setup_game.run_if(resource_exists::<GameAssets>))
        .add_systems(
            Update,
            (
                // Spawn the dialogue runner once the Yarn project has finished compiling
                spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
                // Keep the background properly scaled
                ensure_background_fills_screen,
            ),
        )
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
    windows: Query<&Window>,
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
    if let Ok(window) = windows.get_single() {
        // Get image dimensions
        if let Some(image) = images.get(&game_assets.background) {
            let image_width = image.width() as f32;
            let image_height = image.height() as f32;
            
            // Calculate scale to fill screen while maintaining aspect ratio
            let scale_x = window.resolution.width() / image_width;
            let scale_y = window.resolution.height() / image_height;
            let scale = scale_x.max(scale_y); // Use max to ensure full coverage
            
            if let Ok(mut transform) = background_query.get_single_mut() {
                // Apply smooth scaling with interpolation
                transform.scale = Vec3::new(scale, scale, 1.0);
            }
        }
    }
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}
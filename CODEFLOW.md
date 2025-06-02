# Cosmos-on-the-rocks Code Flow

This document explains the code structure and flow of the Cosmos-on-the-rocks bartending game.

## Overview

Cosmos-on-the-rocks is a bartending game built with the Bevy game engine and YarnSpinner for dialogue management. The game places the player in the role of a bartender in a cosmic setting, serving drinks to various characters while engaging in dialogue.

## Core Components

### 1. Game Initialization (`main.rs`)

The main entry point sets up the Bevy application with necessary plugins:

```rust
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    // Systems...
    .run();
}
```

### 2. Asset Management

Assets are loaded and managed through a resource:

```rust
#[derive(Resource)]
struct GameAssets {
    background: Handle<Image>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_assets = GameAssets {
        background: asset_server.load("dialogue/images/bg.png"),
    };
    commands.insert_resource(game_assets);
}
```

### 3. Scene Setup

The background image is scaled to fill the screen while maintaining aspect ratio:

```rust
fn setup_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) {
    // Add background as a full-screen sprite
    commands.spawn((
        SpatialBundle { /* ... */ },
        BackgroundImage,
        Sprite::from_image(game_assets.background.clone()),
    ));
}
```

### 4. Dialogue System

The dialogue is managed by YarnSpinner, which loads dialogue from Yarn files:

```rust
fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}
```

## Execution Flow

1. **Startup:**
   - Camera is set up
   - Assets are loaded

2. **Update Loop:**
   - Game scene is set up once assets are loaded
   - Dialogue runner is spawned once the Yarn project is compiled
   - Background image is continuously scaled to fill the screen

3. **Dialogue Flow:**
   - Dialogue starts at the "HelloWorld" node
   - Player interacts through dialogue options
   - Dialogue can branch based on player choices

## Future Extensions

- Implement drink mixing mechanics
- Add multiple characters with their own storylines
- Implement effects of drinks on dialogue options
- Add background music and sound effects
- Add day/night cycle system for game progression

## Files Structure

- `main.rs` - Main game code and systems
- `assets/dialogue/hello_world.yarn` - Dialogue script
- `assets/dialogue/images/bg.png` - Background image 
# Project Workflow Documentation

This document outlines the overall structure and key functionalities of the Cosmos-on-the-rocks project.

## Project Structure

- `src/`: Contains all the source code for the application.
  - `engine/`: Core game engine logic.
  - `ui/`: User interface components and logic.
  - `customers/`: Logic related to customer management and interaction.
  - `bar/`: Bar-related functionalities (e.g., drink crafting, orders).
  - `animation/`: Animation related assets and logic.
  - `main.rs`: The main entry point of the application.
  - `constants.rs`: Global constants and configurations.

## Core Modules and Their Functions

### `src/engine`

This module encapsulates the core game engine logic, including state management, asset loading, audio control, and game progression.

#### `GameState` Enum

Defined in `src/engine/mod.rs`, this enum manages the different states of the game:

```rust
pub enum GameState {
    MainMenu,
    Loading,
    Settings,
    CustomerInteraction,
    Crafting,
    EndNight,
}
```

#### `src/engine/asset_loader.rs`

Handles the loading of various game assets. It defines the following asset collections:

- `ImageAssets`: Contains handles to image assets, such as the `bartender.png`.
- `AudioAssets`: Contains handles to audio assets, such as `Ketsa - Drifting Space Jazz.ogg` and `HoliznaCC0 - Space!.ogg`.

#### `src/engine/audio_controller.rs`

Manages in-game audio playback and sound effects.

- `AudioControllerPlugin`: A Bevy plugin for managing audio.
- `play_bg_sound(mut commands: Commands, audio_assets: Res<AudioAssets>)`: A function responsible for playing background audio, demonstrating the use of `SamplePlayer` and audio effects.

#### `src/engine/game_runner.rs`

Orchestrates the main game loop and progression, and sets up essential plugins and game states.

- `GameRunnerPlugin`: A Bevy plugin that:
  - Adds `SeedlingPlugin`, `GameUiPlugin`, and `CustomerPlugin`.
  - Sets the background clear color.
  - Configures the `LoadingState` to load `AudioAssets` and `ImageAssets`, transitioning to `GameState::CustomerInteraction` upon completion.
  - Adds the `setup_camera` function to the `Startup` system.
- `setup_camera(mut commands: Commands)`: Spawns a default 2D camera for the game.

### `src/ui`

This module handles the user interface elements and their interactions.

#### `GameUiPlugin`

Defined in `src/ui/mod.rs`, this Bevy plugin manages UI-related systems, specifically for the `MainMenu` game state:

- It calls `setup_main_menu` when entering `GameState::MainMenu`.
- It runs `button_interaction_system` during the `Update` stage when in `GameState::MainMenu`.
- It calls `cleanup_menu` when exiting `GameState::MainMenu`.

#### `src/ui/main_menu.rs`

This file contains the logic for the main menu, including its setup, button interactions, and cleanup.

- `MenuButtons` Enum: Defines the types of buttons available in the main menu (`Play`, `Settings`).
- `OnMainMenuScreen` Component: A marker component used to identify and manage entities belonging to the main menu screen.
- `setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>)`: Initializes the main menu UI, including loading assets, creating the layout, and spawning the title image and interactive buttons (`Play Game`, `Settings`).
- `button_interaction_system`: A system that processes user interactions with the main menu buttons, changing their visual state (hovered/normal) and transitioning the `GameState` when a button is pressed (`GameState::Loading` for Play, `GameState::Settings` for Settings).
- `cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>)`: Despawns all entities associated with the main menu screen when the game transitions out of `GameState::MainMenu`.

### `src/customers`

This module handles customer-related logic, including their attributes, interactions, and spawning.

#### `CustomerPlugin`

Defined in `src/customers/mod.rs`, this Bevy plugin:

- Adds the `DialogPlugin`.
- Calls `spawn_customer` and `play_bg_sound` when entering `GameState::CustomerInteraction`.

#### `Customer` Struct

Represents a customer in the game with the following fields:

- `name`: The customer's name.
- `emotional_state`: The customer's current emotional state (e.g., `Happy`, `Sad`).
- `faction`: The customer's faction (e.g., `Rebels`, `Empire`).
- `likes`: A list of things the customer likes.
- `dislikes`: A list of things the customer dislikes.

#### `EmotionalState` Enum

An enum defined in `src/customers/mod.rs` to represent a customer's emotional state:

```rust
pub enum EmotionalState {
    Happy,
    Sad,
    Angry,
    Neutral,
}
```

#### `Factions` Enum

An enum defined in `src/customers/mod.rs` to represent different factions:

```rust
pub enum Factions {
    Rebels,
    Empire,
    BountyHunter,
    Unknown,
}
```

#### `spawn_customer(mut commands: Commands, image_assets: Res<ImageAssets>, mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>)`

A function responsible for spawning a customer entity into the game, including their sprite, animation state, and initial position.

#### `src/customers/dialogue.rs`

This file contains the logic for customer dialogue and interaction.

- `DialogPlugin`: A Bevy plugin that:
  - Adds `YarnSpinnerPlugin` and `ExampleYarnSpinnerDialogueViewPlugin` for dialogue management.
  - Runs the `spawn_dialogue_runner` system when a `YarnProject` resource is added and the game is in `GameState::CustomerInteraction`.
- `spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>)`: Spawns a dialogue runner, initializes it with a `YarnProject`, and starts the "HelloWorld" node.

### `src/bar`

This module is intended to contain logic related to bar functionalities, such as drink crafting and order management. It is currently empty.

### `src/animation`

This module is intended to contain animation-related assets and logic. It is currently empty.

### `src/main.rs`

This is the main entry point of the application, responsible for setting up the Bevy application and its plugins.

- `main()`: The primary function that:
  - Creates a new Bevy `App`.
  - Adds `DefaultPlugins` with a custom `WindowPlugin`.
  - Adds the `GameRunnerPlugin`.
  - Initializes the `GameState`.
  - Sets the initial `ClearColor` for the background.
  - Runs the application.
- `create_window_plugin()`: A helper function that configures the primary game window, setting its title to "Cosmos on the Rocks".

### `src/constants.rs`

This file contains global constants and configurations for the application, primarily defining various `Color` values used for UI elements and other visual components:

- `NORMAL_BUTTON`
- `HOVERED_BUTTON`
- `INPUT_FIELD_BUTTON`
- `INPUT_FIELD_BG`
- `WHITE`
- `BUTTON_BORDER`
- `TEXT_COLOR`
- `RED`
- `BLUE`
- `GREEN` 
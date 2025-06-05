# Cosmos on the Rocks - Game Workflow

## Overview

Cosmos on the Rocks is a bartending simulation game set in a space-themed bar where the player interacts with various patrons, crafts drinks, and manages patron emotions. The game combines dialogue systems, emotion management, and drink crafting mechanics to create an engaging narrative experience.

## Core Systems

### 1. Game State Management

The game uses a state machine to control the flow between different game modes:

- **MainMenu**: The entry point of the game
- **Loading**: Resource loading state
- **CustomerInteraction**: Main gameplay where players talk with patrons
- **Crafting**: Interface for creating drinks
- **Settings**: Game configuration options
- **EndNight**: End of a game session

States are managed through the `GameState` enum and transitions are handled by the `NextState<GameState>` resource.

### 2. Patron System

Patrons are the characters that visit the bar and interact with the player. Each patron has:

- **Identity**: Name and personality traits
- **Emotions**: Three emotional dimensions (Happiness, Sadness, Anger)
- **Dialogue**: Unique conversation trees using the YarnSpinner system

The main patrons include:
- **Zara**: Security worker for Galactic Corp
- **Kael**: Another patron with unique traits
- **Unit 734**: Possibly a robotic or synthetic character
- **Lyra**: Another patron with unique traits

### 3. Emotion System

Each patron has three emotional states that can be influenced by player actions:

- **Happiness**: Positive emotion (0-100%)
- **Sadness**: Melancholic emotion (0-100%)
- **Anger**: Negative emotion (0-100%)

The emotion system is central to gameplay, as patrons' emotions affect dialogue options and outcomes.

### 4. Dialogue System

The game uses YarnSpinner, a narrative scripting language, to manage conversations:

- **Dialogue Nodes**: Segments of conversation with branching paths
- **Dialogue Options**: Player-selectable responses
- **Dialogue Conditions**: Rules that determine when certain options are available
- **Dialogue Effects**: Actions triggered when options are selected

Dialogue files are stored in `.yarn` format and are compiled at runtime.

### 5. Drink Crafting System

The player can craft drinks that affect patron emotions:

- **Drink Types**: Different drinks with varying effects
- **Crafting Interface**: UI for creating drinks
- **Emotion Effects**: How drinks modify patron emotions

Each drink can have different effects on happiness, sadness, and anger levels.

### 6. Intel System

Throughout conversations, players can discover "intel" - pieces of information about the world and characters:

- **Intel Registry**: Tracks discovered information
- **Intel Effects**: Some dialogue options are only available when certain intel has been discovered

## Game Flow

### 1. Starting the Game

1. The game begins in the **MainMenu** state
2. Player selects "Play Game"
3. Game transitions to the **Loading** state
4. Resources (assets, dialogue files, etc.) are loaded
5. Game spawns patrons and initializes their emotional states
6. Game transitions to the **CustomerInteraction** state

### 2. Customer Interaction Phase

1. A dialogue is initiated with a patron (default is "ZaraDialogue")
2. Dialogue runner displays the patron's lines and player's response options
3. Player selects dialogue options which may:
   - Progress the narrative
   - Affect patron emotions
   - Discover new intel
   - Trigger a state change (e.g., to Crafting)

### 3. Crafting Phase

1. Game transitions to the **Crafting** state when triggered by dialogue
2. The crafting interface appears with options to create drinks
3. Player makes drink decisions that will affect patron emotions
4. When crafting is complete, the "Continue" button returns to **CustomerInteraction**

### 4. Emotion Management

Throughout the game, the player can:
- View patron emotions by pressing 'D'
- Select Zara as the active patron by pressing '1'
- Give Zara a drink that increases anger by pressing 'G'
- Test general drink effects by pressing 'T'

### 5. Ending the Session

When the narrative reaches its conclusion, the game may transition to the **EndNight** state.

## Technical Implementation

### Dialogue Implementation

Dialogue is defined in Yarn files and processed by the YarnSpinner system:
- `zara.yarn`: Contains dialogue nodes for Zara
- `start.yarn`: Contains initial dialogue
- `hello_world.yarn`: Contains bartender monologue

Key nodes include:
- `ZaraDialogue`: Initial conversation
- `ZaraOrder`: Drink ordering segment
- `ZaraDay`: Discussion about Zara's day
- `ZaraSecret`: Reveals secretive information
- `ZaraWait`: Transition to crafting
- `ZaraAnnoyance`: Triggered by certain responses

### Emotion UI

The emotion UI system allows players to:
- See which patron is currently selected
- View the emotional state of the selected patron
- Track changes in emotions as drinks are served

### Crafting Menu

The crafting menu system:
- Displays the current drink being crafted
- Shows effects the drink will have
- Provides a "Continue" button to serve the drink
- Automatically applies drink effects to patrons

### Drink Effects

Drinks modify patron emotions through the `EmotionDrink` component:
- `happiness_effect`: Change to happiness level
- `sadness_effect`: Change to sadness level
- `anger_effect`: Change to anger level
- `target_patron`: Which patron receives the effects

## Debugging Tools

The game includes several debugging features:
- Console logging of patron emotions
- Keyboard shortcuts for testing drink effects
- Information about available dialogue nodes

## Development Notes

- The UI components use Bevy's UI system for layout and interaction
- YarnSpinner dialogue files should be placed in the "dialogue" directory
- The game uses a reactive event system for handling dialogue interactions
- Patron emotions persist throughout the game session
- Dialogue conditions can check emotional states and discovered intel

## Key Keyboard Controls

- **1**: Select Zara as the active patron
- **D**: Display the selected patron's emotions
- **G**: Give Zara a drink (+5 anger)
- **T**: Test general drink effects (for development)

## File Structure

- `src/ui/`: UI components and systems
  - `emotion_ui.rs`: Emotion display and interaction
  - `crafting_menu.rs`: Drink crafting interface
- `src/customers/`: Patron-related systems
  - `dialogue.rs`: Dialogue handling and drink effects
- `src/dialogue/`: Core dialogue systems
  - `systems.rs`: Dialogue interaction logic
  - `patrons.rs`: Patron component definitions
- `src/engine/`: Core game systems
  - `game_runner.rs`: Main game loop and state management

## Game Loop Summary

1. Start in main menu
2. Enter customer interaction mode
3. Talk with patrons through dialogue system
4. Select dialogue options affecting emotions and narrative
5. Enter crafting mode when triggered
6. Create drinks with emotional effects
7. Return to customer interaction
8. Repeat steps 3-7 until narrative conclusion
9. End night
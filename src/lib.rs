use bevy::prelude::*;

pub mod dialogue;

use dialogue::{DialoguePlugin, IntelPlugin, DialogueInteractionPlugin};

/// Main plugin for Cosmos on the Rocks game
pub struct CosmosOnTheRocksPlugin;

impl Plugin for CosmosOnTheRocksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DialoguePlugin)
            .add_plugins(IntelPlugin)
            .add_plugins(DialogueInteractionPlugin);
    }
}

/// Game system initialization
pub fn app() -> App {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins)
       .add_plugins(CosmosOnTheRocksPlugin);
    
    app
} 
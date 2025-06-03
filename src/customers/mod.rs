use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{customers::dialogue::spawn_dialogue_runner, engine::GameState};

pub mod dialogue;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((YarnSpinnerPlugin::new(),
            ExampleYarnSpinnerDialogueViewPlugin::new(),))
            .add_systems(
            Update,
            spawn_dialogue_runner
            .run_if(in_state(GameState::CustomerInteraction))
            .run_if(resource_added::<YarnProject>),
        );
    }
}
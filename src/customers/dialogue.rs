use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnProject;

pub fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}

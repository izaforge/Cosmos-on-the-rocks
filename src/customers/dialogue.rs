use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnProject;

pub fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    println!("Spawning dialogue runner with project: {:?}", project);
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}

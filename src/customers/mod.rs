use bevy::prelude::*;


use crate::{customers::dialogue::{spawn_dialogue_runner, DialogPlugin}, engine::{asset_loader::ImageAssets, audio_controller::play_bg_sound, GameState}};

pub mod dialogue;

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DialogPlugin)
        .add_systems(
                OnEnter(GameState::CustomerInteraction),
                (spawn_customer, play_bg_sound),
            );
    }
}

fn spawn_customer(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Sprite::from_image(image_assets.bartender.clone()),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
    ));
}
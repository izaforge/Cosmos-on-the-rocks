use bevy::prelude::*;
use crate::constants::TEXT_COLOR;

#[derive(Component)]
pub struct OnCraftingScreen;

pub fn setup_crafting_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            OnCraftingScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Crafting State (Placeholder)"),
                TextFont {
                    font: menu_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

pub fn cleanup_crafting_menu(mut commands: Commands, query: Query<Entity, With<OnCraftingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
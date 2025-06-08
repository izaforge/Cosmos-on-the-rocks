use crate::{
    bar::crafting::OnCraftingScreen, constants::{BUTTON_BORDER, HOVERED_BUTTON, NORMAL_BUTTON, TEXT_COLOR}, engine::GameState
};
use bevy::prelude::*;

#[derive(Component)]
pub enum CraftingButtons {
    Craft,
    Reset,
}

pub fn setup_crafting_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");

    commands
        .spawn((
            Node {
                align_content: AlignContent::Center,
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Center,
                position_type: PositionType::Relative,
                flex_wrap: FlexWrap::NoWrap,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                margin: UiRect::bottom(Val::Px(50.0)),
                ..Default::default()
            },
            OnCraftingScreen,
        ))
        .with_children(|parent| {
            // Craft Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .insert(CraftingButtons::Craft)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Craft"),
                        TextFont {
                            font: menu_font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });

            // Reset Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .insert(CraftingButtons::Reset)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Reset"),
                        TextFont {
                            font: menu_font,
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

// System to handle crafting button interactions
pub fn crafting_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CraftingButtons),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match button {
                CraftingButtons::Craft => {
                    println!("Craft Button Clicked");
                    // Implement crafting logic here
                }
                CraftingButtons::Reset => {
                    println!("Reset Button Clicked");
                    // Implement reset logic here
                }
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Cleanup system for when exiting the crafting state
pub fn cleanup_crafting_menu(
    mut commands: Commands,
    query: Query<Entity, With<OnCraftingScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

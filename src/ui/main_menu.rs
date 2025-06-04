use crate::{
    constants::{BUTTON_BORDER, HOVERED_BUTTON, NORMAL_BUTTON, TEXT_COLOR},
    engine::GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButtons {
    Play,
    Settings,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");
    let title_img = asset_server.load("images/name.png");
    commands
        .spawn((
            Node {
                align_content: AlignContent::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                position_type: PositionType::Relative,
                flex_wrap: FlexWrap::NoWrap,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // Title Image
            parent.spawn((
                ImageNode {
                    image: title_img,
                    ..default()
                },
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(200.0),
                    margin: UiRect::top(Val::VMin(5.)),
                    ..default()
                },
            ));
            // Start Game Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .insert(MenuButtons::Play)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Play Game"),
                        TextFont {
                            font: menu_font.clone(),
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
            // Game Settings Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .insert(MenuButtons::Settings)
                .with_children(|parent| {
                    parent.spawn((
                        Text::from("Settings"),
                        TextFont {
                            font: menu_font,
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

// System to handle button interaction
pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match button {
                MenuButtons::Play => {
                    println!("Play Game Button Clicked");
                    game_state.set(GameState::Loading);
                }
                MenuButtons::Settings => {
                    println!("Settings Button Clicked");
                    game_state.set(GameState::Crafting);
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

// System to cleanup menu when exiting MainMenu state
pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

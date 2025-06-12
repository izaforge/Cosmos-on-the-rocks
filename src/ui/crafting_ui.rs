use crate::{
    bar::{
        crafting::OnCraftingScreen,
        drinks::{spawn_crafted_drink, Drink},
        glass::Glass,
    },
    constants::{BUTTON_BORDER, HOVERED_BUTTON, NORMAL_BUTTON, TEXT_COLOR},
    engine::{asset_loader::ImageAssets},
};
use bevy::prelude::*;

#[derive(Component)]
pub enum CraftingButtons {
    Craft,
    Reset,
}

#[derive(Component)]
pub struct GlassDetailsUI;

#[derive(Component)]
pub struct DrinkSprite;

pub fn setup_glass_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Glass>,
) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");
    info!("Crafting Menu Font Loaded: {:?}", menu_font);
    if let Some(glass) = query.iter().next() {
        info!("Crafting Menu Glass: {:#?}", glass);
        commands.spawn((
            Text::new(format!(
                "Volume: {:.1} / {:.1}\nTaste: {:#?}\nEffects: {:#?}",
                glass.get_current_volume(),
                glass.capacity,
                glass.taste,
                glass.effect,
            )),
            Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            BorderColor(Color::srgb(0.3, 0.2, 0.1)),
            BorderRadius::ZERO,
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            TextFont {
                font: menu_font.clone(),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            OnCraftingScreen,
            GlassDetailsUI,
        ));
    }
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
                            font: menu_font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

pub fn crafting_button_interaction_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CraftingButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut glass_query: Query<&mut Glass>,
    drink_query: Query<Entity, With<Drink>>,
    drink_sprite_query: Query<Entity, With<DrinkSprite>>,
    image_assets: Res<ImageAssets>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match button {
                CraftingButtons::Craft => {
                    for entity in drink_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    for glass in glass_query.iter_mut() {
                        let drink = Drink::from(glass.clone());
                        info!("Crafted {:#?}", drink);
                        spawn_crafted_drink(&mut commands, drink, &image_assets);
                    }
                }
                CraftingButtons::Reset => {
                    for entity in drink_sprite_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    for mut glass in glass_query.iter_mut() {
                        glass.reset();
                        info!("Glass Reset {:#?}", glass);
                    }
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

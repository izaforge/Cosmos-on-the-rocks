use crate::{
    bar::{
        crafting::OnCraftingScreen,
        drinks::{CreatedDrink, Drink},
        glass::Glass,
    },
    constants::{BUTTON_BORDER, HOVERED_BUTTON, NORMAL_BUTTON, TEXT_COLOR},
    engine::{GameState, asset_loader::ImageAssets},
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
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CraftingButtons),
        (Changed<Interaction>, With<Button>),
    >,
    mut glass_query: Query<&mut Glass>,
    image_assets: Res<ImageAssets>,
) {
    for (interaction, mut color, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match button {
                CraftingButtons::Craft => {
                    for glass in glass_query.iter_mut() {
                        let drink = Drink::from(glass.clone());
                        info!("Crafted {:#?}", drink);
                        // Choose the correct image for the drink
                        let drink_image = match drink.created_drink {
                            CreatedDrink::ZeroPhase => image_assets.zero_phase.clone(),
                            CreatedDrink::CryoDrop => image_assets.cryo_drop.clone(),
                            CreatedDrink::StellarLumen => image_assets.stellar_lumen.clone(),
                            CreatedDrink::Cosmopolitan => image_assets.cosmopolitan.clone(),
                            CreatedDrink::SynthCascade => image_assets.synth_cascade.clone(),
                            CreatedDrink::OldMemory => image_assets.old_memory.clone(),
                            CreatedDrink::EchoBloom => image_assets.echo_bloom.clone(),
                            CreatedDrink::BotanicalSurge => image_assets.botanica_surge.clone(),
                            CreatedDrink::BinaryBarrel => image_assets.binary_barrel.clone(),
                            CreatedDrink::EventHorizon => image_assets.event_horizon.clone(),
                        };

                        // Spawn the crafted drink as a Sprite with the Drink component
                        commands.spawn((
                            drink,
                            Sprite {
                                image: drink_image,
                                custom_size: Some(Vec2::new(256., 256.)),
                                ..Default::default()
                            },
                            Transform::from_xyz(0.0, 0.0, 2.0),
                            Pickable::default(),
                        )).observe(
            |_: Trigger<Pointer<Click>>,
             mut game_state: ResMut<NextState<GameState>>,
             | {
                game_state.set(GameState::Dialogues);
            },
        );
                    }
                }
                CraftingButtons::Reset => {
                    println!("Reset Button Clicked");
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

// Cleanup system for when exiting the crafting state
pub fn cleanup_crafting_menu(
    mut commands: Commands,
    query: Query<Entity, With<OnCraftingScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

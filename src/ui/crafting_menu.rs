use bevy::prelude::*;
use crate::constants::TEXT_COLOR;
use crate::engine::GameState;

#[derive(Component)]
pub struct OnCraftingScreen;

#[derive(Component)]
pub struct ContinueButton;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_crafting_interaction.run_if(in_state(GameState::Crafting)));
    }
}

pub fn setup_crafting_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_font = asset_server.load("fonts/Nasa21.ttf");
    
    // First give Zara a drink that increases her anger
    give_zara_anger_drink(&mut commands);

    commands
        .spawn((
            OnCraftingScreen,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Crafting a Strong Drink for Zara"),
                TextFont {
                    font: menu_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
            
            // Description
            parent.spawn((
                Text::new("You've crafted a potent drink that will increase Zara's anger by 5."),
                TextFont {
                    font: menu_font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Continue button
            parent.spawn((
                Button,
                ContinueButton,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Continue"),
                    TextFont {
                        font: menu_font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

/// Function to give Zara a drink that increases her anger
fn give_zara_anger_drink(commands: &mut Commands) {
    info!("Crafting a strong drink for Zara (+5 anger)");
    
    // Spawn a drink entity that will be processed by the handle_drink_effects system
    commands.spawn(crate::customers::dialogue::EmotionDrink {
        happiness_effect: 0,
        sadness_effect: 0,
        anger_effect: 5,
        target_patron: None,  // We'll find Zara by name in the drink system
    });
}

/// System to handle button clicks in the crafting menu
fn handle_crafting_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            info!("Continue button pressed. Returning to customer interaction.");
            
            // Add a command to start the ZaraAfterDrink node when dialogue resumes
            commands.insert_resource(crate::customers::dialogue::NextDialogueNode("ZaraAfterDrink".to_string()));
            
            // Transition back to CustomerInteraction state
            next_state.set(GameState::CustomerInteraction);
        }
    }
}

pub fn cleanup_crafting_menu(
    mut commands: Commands,
    query: Query<Entity, With<OnCraftingScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
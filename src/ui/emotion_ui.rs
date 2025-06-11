use crate::{
    constants::TEXT_COLOR,
    customers::{Customer, CustomerAnger, CustomerHappiness, CustomerSadness, OnCustomerScreen},
    engine::GameState,
};
use bevy::prelude::*;

/// Component marker for selected patron whose emotions should be displayed
#[derive(Component)]
pub struct SelectedPatron;

/// Component marker for emotion UI elements
#[derive(Component)]
pub struct EmotionUiDisplay;

/// Plugin for emotion UI systems
pub struct EmotionUiPlugin;

impl Plugin for EmotionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dialogues), setup_emotion_ui)
            .add_systems(
                Update,
                (update_emotion_ui, update_selected_patron).run_if(in_state(GameState::Dialogues)),
            )
            .add_systems(OnExit(GameState::Dialogues), cleanup_emotion_ui);
    }
}

/// System to create the emotion UI display when entering dialogue state
fn setup_emotion_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Nasa21.ttf");

    commands
        .spawn((
            EmotionUiDisplay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                min_width: Val::Px(200.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Customer Emotions"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Patron name
            parent
                .spawn((
                    Text::new("No customer selected"),
                    TextFont {
                        font: font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                ))
                .insert(PatronNameText);

            // Happiness
            parent
                .spawn((
                    Text::new("Happiness: 0%"),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 0.2)),
                ))
                .insert(HappinessText);

            // Sadness
            parent
                .spawn((
                    Text::new("Sadness: 0%"),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.8)),
                ))
                .insert(SadnessText);

            // Anger
            parent
                .spawn((
                    Text::new("Anger: 0%"),
                    TextFont {
                        font: font.clone(),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.2, 0.2)),
                ))
                .insert(AngerText);
        });
}

/// Component markers for different emotion text elements
#[derive(Component)]
struct PatronNameText;

#[derive(Component)]
struct HappinessText;

#[derive(Component)]
struct SadnessText;

#[derive(Component)]
struct AngerText;

/// System to update the emotion UI display
fn update_emotion_ui(
    selected_patron_query: Query<
        (
            &Customer,
            &CustomerHappiness,
            &CustomerSadness,
            &CustomerAnger,
        ),
        With<SelectedPatron>,
    >,
    mut name_text_query: Query<
        &mut Text,
        (
            With<PatronNameText>,
            Without<HappinessText>,
            Without<SadnessText>,
            Without<AngerText>,
        ),
    >,
    mut happiness_text_query: Query<
        &mut Text,
        (
            With<HappinessText>,
            Without<PatronNameText>,
            Without<SadnessText>,
            Without<AngerText>,
        ),
    >,
    mut sadness_text_query: Query<
        &mut Text,
        (
            With<SadnessText>,
            Without<PatronNameText>,
            Without<HappinessText>,
            Without<AngerText>,
        ),
    >,
    mut anger_text_query: Query<
        &mut Text,
        (
            With<AngerText>,
            Without<PatronNameText>,
            Without<HappinessText>,
            Without<SadnessText>,
        ),
    >,
) {
    if let Ok((customer, happiness, sadness, anger)) = selected_patron_query.single() {
        if let Ok(mut text) = name_text_query.single_mut() {
            text.0 = format!("Customer: {}", customer.name);
        }
        if let Ok(mut text) = happiness_text_query.single_mut() {
            text.0 = format!("Happiness: {}%", happiness.value);
        }
        if let Ok(mut text) = sadness_text_query.single_mut() {
            text.0 = format!("Sadness: {}%", sadness.value);
        }
        if let Ok(mut text) = anger_text_query.single_mut() {
            text.0 = format!("Anger: {}%", anger.value);
        }
    } else {
        if let Ok(mut text) = name_text_query.single_mut() {
            text.0 = "No customer selected".to_string();
        }
        if let Ok(mut text) = happiness_text_query.single_mut() {
            text.0 = "Happiness: 0%".to_string();
        }
        if let Ok(mut text) = sadness_text_query.single_mut() {
            text.0 = "Sadness: 0%".to_string();
        }
        if let Ok(mut text) = anger_text_query.single_mut() {
            text.0 = "Anger: 0%".to_string();
        }
    }
}

/// System to handle selecting customers (automatically select Zara)
fn update_selected_patron(
    mut commands: Commands,
    customers: Query<(Entity, &Customer), (With<OnCustomerScreen>, Without<SelectedPatron>)>,
    selected: Query<Entity, With<SelectedPatron>>,
) {
    // Auto-select Zara on startup if no customer is currently selected
    if selected.is_empty() {
        // Find Zara and select her
        for (entity, customer) in customers.iter() {
            if customer.name == "Zara" {
                commands.entity(entity).insert(SelectedPatron);
                info!("Auto-selected customer: Zara");
                break;
            }
        }
    }
}

/// System to cleanup emotion UI when exiting dialogue state
fn cleanup_emotion_ui(mut commands: Commands, query: Query<Entity, With<EmotionUiDisplay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

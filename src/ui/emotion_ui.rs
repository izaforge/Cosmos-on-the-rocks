use bevy::prelude::*;
use crate::dialogue::patrons::{Patron, Happiness, Sadness, Anger};

/// Component marker for selected patron whose emotions should be displayed
#[derive(Component)]
pub struct SelectedPatron;

/// Plugin for emotion UI systems
pub struct EmotionUiPlugin;

impl Plugin for EmotionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_emotion_ui, update_selected_patron));
    }
}

/// System to update the emotion UI using console logs
fn update_emotion_ui(
    selected_patron_query: Query<(&Patron, &Happiness, &Sadness, &Anger), With<SelectedPatron>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Only show emotions when D key is pressed (to avoid spamming the console)
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        if let Ok((patron, happiness, sadness, anger)) = selected_patron_query.single() {
            info!("------- PATRON EMOTIONS -------");
            info!("Patron: {}", patron.name);
            info!("Happiness: {}%", happiness.value);
            info!("Sadness: {}%", sadness.value);
            info!("Anger: {}%", anger.value);
            info!("------------------------------");
        } else {
            info!("No patron selected. Press 1-4 to select a patron.");
        }
    }
}

/// System to handle selecting a new patron
fn update_selected_patron(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    patrons: Query<(Entity, &Patron)>,
    selected: Query<Entity, With<SelectedPatron>>,
) {
    // Just a simple example - press 1, 2, 3, 4 to select different patrons
    let mut selection = None;
    
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        // Find Zara
        for (entity, patron) in patrons.iter() {
            if patron.name == "Zara" {
                selection = Some(entity);
                info!("Selected patron: Zara");
                break;
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        // Find Kael
        for (entity, patron) in patrons.iter() {
            if patron.name == "Kael" {
                selection = Some(entity);
                info!("Selected patron: Kael");
                break;
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        // Find Unit 734
        for (entity, patron) in patrons.iter() {
            if patron.name == "Unit 734" {
                selection = Some(entity);
                info!("Selected patron: Unit 734");
                break;
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        // Find Lyra
        for (entity, patron) in patrons.iter() {
            if patron.name == "Lyra" {
                selection = Some(entity);
                info!("Selected patron: Lyra");
                break;
            }
        }
    }
    
    // Update selection if needed
    if let Some(selection) = selection {
        // Remove old selection
        for entity in selected.iter() {
            commands.entity(entity).remove::<SelectedPatron>();
        }
        
        // Add new selection
        commands.entity(selection).insert(SelectedPatron);
    }
} 
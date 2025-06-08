use crate::dialogue::patrons::{Anger, Happiness, Patron, Sadness};
use bevy::prelude::*;

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
) {
    if let Ok((patron, happiness, sadness, anger)) = selected_patron_query.single() {
        println!("------- PATRON EMOTIONS -------");
        println!("Patron: {}", patron.name);
        println!("Happiness: {}%", happiness.value);
        println!("Sadness: {}%", sadness.value);
        println!("Anger: {}%", anger.value);
        println!("------------------------------");
    }
}

/// System to handle selecting Zara (automatically)
fn update_selected_patron(
    mut commands: Commands,
    patrons: Query<(Entity, &Patron)>,
    selected: Query<Entity, With<SelectedPatron>>,
) {
    // Auto-select Zara on startup if no patron is currently selected
    if selected.is_empty() {
        // Find Zara and select her
        for (entity, patron) in patrons.iter() {
            if patron.name == "Zara" {
                commands.entity(entity).insert(SelectedPatron);
                println!("Auto-selected patron: Zara");
                break;
            }
        }
    }
}

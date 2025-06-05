use bevy::prelude::*;
use crate::dialogue::patrons::{Patron, Happiness, Sadness, Anger};

/// Component marker for selected patron whose emotions should be displayed
#[derive(Component)]
pub struct SelectedPatron;

/// Resource to track if Zara has been auto-selected
#[derive(Resource)]
struct ZaraSelected(bool);

/// Plugin for emotion UI systems
pub struct EmotionUiPlugin;

impl Plugin for EmotionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_emotion_ui, update_selected_patron, give_zara_drink))
           .init_resource::<ZaraSelected>(); // Initialize the resource
        info!("EmotionUiPlugin initialized!"); // Debug log to confirm plugin loading
        info!("=====================================================");
        info!("Emotion UI Controls:");
        info!("  Zara is now automatically selected");
        info!("  Press KeyD (D) to show Zara's emotions");
        info!("  Press KeyG (G) to give Zara a drink (+5 anger)");
        info!("  Press KeyT (T) to test general drink effects");
        info!("=====================================================");
    }
}

impl Default for ZaraSelected {
    fn default() -> Self {
        ZaraSelected(false)
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
            info!("No patron selected. Zara should be auto-selected.");
        }
    }
}

/// System to give Zara a drink that increases anger by 5
fn give_zara_drink(
    mut patron_query: Query<(&Patron, &mut Happiness, &mut Sadness, &mut Anger), With<SelectedPatron>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if let Ok((patron, _happiness, _sadness, mut anger)) = patron_query.single_mut() {
            if patron.name == "Zara" {
                // Increase anger by 5, clamping to max 100
                anger.value = (anger.value + 5).min(100);
                info!("Gave Zara a drink! Anger increased to {}%", anger.value);
            }
        } else {
            info!("Zara not selected. Will attempt to auto-select her.");
        }
    }
}

/// System to handle selecting Zara (automatically and when pressing 1)
fn update_selected_patron(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    patrons: Query<(Entity, &Patron)>,
    selected: Query<Entity, With<SelectedPatron>>,
    mut zara_selected: ResMut<ZaraSelected>,
) {
    // Auto-select Zara on startup if not already selected
    if !zara_selected.0 {
        // Find Zara and select her
        for (entity, patron) in patrons.iter() {
            if patron.name == "Zara" {
                // Remove old selection if any
                for entity in selected.iter() {
                    commands.entity(entity).remove::<SelectedPatron>();
                }
                
                // Add new selection
                commands.entity(entity).insert(SelectedPatron);
                info!("Auto-selected patron: Zara");
                zara_selected.0 = true;
                break;
            }
        }
    }

    // Still keep manual selection ability with key 1
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        info!("Key 1 pressed");
        
        // Find Zara
        for (entity, patron) in patrons.iter() {
            if patron.name == "Zara" {
                // Remove old selection
                for entity in selected.iter() {
                    commands.entity(entity).remove::<SelectedPatron>();
                }
                
                // Add new selection
                commands.entity(entity).insert(SelectedPatron);
                info!("Selected patron: Zara");
                info!("Selection updated successfully");
                break;
            }
        }
    }
} 
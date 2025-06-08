use crate::bar::ingredient::PrimaryEffect;
use crate::customers::dialogue::PatronEffects;
use crate::engine::GameState;
use bevy::prelude::*;
use bevy::ui::UiSystem;

/// Plugin for displaying mood/effect parameters UI
pub struct MoodUiPlugin;

impl Plugin for MoodUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dialogues), setup_mood_ui)
            .add_systems(
                Update,
                update_mood_display.run_if(in_state(GameState::Dialogues)),
            )
            .add_systems(OnExit(GameState::Dialogues), cleanup_mood_ui);
    }
}

/// Component marker for mood UI elements
#[derive(Component)]
pub struct MoodUiDisplay;

/// System to create the mood UI display when entering customer interaction
fn setup_mood_ui(mut commands: Commands, game_state: Res<State<GameState>>) {
    // Always show the mood UI when in CustomerInteraction state
    if game_state.get() == &GameState::Dialogues {
        commands
            .spawn((
                MoodUiDisplay,
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
                    Text::new("Current Mood"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Effect displays
                for effect_type in [
                    PrimaryEffect::Energizing,
                    PrimaryEffect::Calming,
                    PrimaryEffect::TruthInducing,
                    PrimaryEffect::MindEnhancing,
                ] {
                    parent.spawn((
                        Text::new(format!("{:?}: 0", effect_type)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        EffectValueText(effect_type),
                    ));
                }
            });
    }
}

/// Component to identify effect value text elements
#[derive(Component)]
struct EffectValueText(PrimaryEffect);

/// System to update the mood display when effects change
fn update_mood_display(
    patron_effects: Res<PatronEffects>,
    mut text_query: Query<(&mut Text, &mut TextColor, &EffectValueText)>,
) {
    // Only update if PatronEffects has changed
    if !patron_effects.is_changed() {
        return;
    }

    // Update text content and color based on effect values
    for (mut text, mut text_color, effect_text) in text_query.iter_mut() {
        let value = patron_effects.effects.get(&effect_text.0).unwrap_or(&0);
        **text = format!("{:?}: {}", effect_text.0, value);

        // Update text color based on value (higher values = brighter colors)
        let intensity = (*value as f32) / 10.0;
        match effect_text.0 {
            PrimaryEffect::Energizing => {
                text_color.0 = Color::srgb(1.0, 0.5 + intensity * 0.5, 0.5 + intensity * 0.5);
            }
            PrimaryEffect::Calming => {
                text_color.0 = Color::srgb(0.5 + intensity * 0.5, 0.5 + intensity * 0.5, 1.0);
            }
            PrimaryEffect::TruthInducing => {
                text_color.0 = Color::srgb(0.5 + intensity * 0.5, 1.0, 0.5 + intensity * 0.5);
            }
            PrimaryEffect::MindEnhancing => {
                text_color.0 = Color::srgb(1.0, 0.5 + intensity * 0.5, 1.0);
            }
            _ => {
                text_color.0 = Color::srgb(0.8, 0.8, 0.8);
            }
        }
    }
}

/// System to clean up mood UI when exiting customer interaction
fn cleanup_mood_ui(mut commands: Commands, ui_query: Query<Entity, With<MoodUiDisplay>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
    info!("Cleaned up mood UI");
}

use bevy::prelude::*;
use super::nodes::{self, DialogueTree, DialogueCondition, DialogueEffect};
use super::intel::*;
use super::patrons::{Patron as PatronComponent, Mood as PatronMood};

/// Component that marks an entity as having an active dialogue
#[derive(Component)]
pub struct ActiveDialogue {
    pub tree_handle: Handle<DialogueTree>,
    pub current_node_id: String,
}

/// Component for dialogue option entities
#[derive(Component)]
pub struct DialogueOptionButton {
    pub option_index: usize,
}

/// Event for when a dialogue option is selected
#[derive(Event)]
pub struct DialogueOptionSelected {
    pub dialogue_entity: Entity,
    pub option_index: usize,
}

/// Plugin for dialogue interaction systems
pub struct DialogueInteractionPlugin;

impl Plugin for DialogueInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DialogueOptionSelected>()
           .add_systems(Update, (
               handle_dialogue_option_selection,
               update_dialogue_ui,
           ));
    }
}

/// System to handle dialogue option selection and apply effects
fn handle_dialogue_option_selection(
    _commands: Commands,
    mut dialogue_query: Query<(Entity, &mut ActiveDialogue)>,
    dialogue_assets: Res<Assets<DialogueTree>>,
    mut intel_registry: ResMut<IntelRegistry>,
    mut intel_events: EventWriter<IntelDiscoveredEvent>,
    mut dialogue_events: EventReader<DialogueOptionSelected>,
) {
    for event in dialogue_events.read() {
        if let Ok((_entity, mut active_dialogue)) = dialogue_query.get_mut(event.dialogue_entity) {
            if let Some(dialogue_tree) = dialogue_assets.get(&active_dialogue.tree_handle) {
                if let Some(current_node) = dialogue_tree.nodes.get(&active_dialogue.current_node_id) {
                    // Make sure the option index is valid
                    if event.option_index < current_node.options.len() {
                        let selected_option = &current_node.options[event.option_index];
                        
                        // Apply effects
                        for effect in &selected_option.effects {
                            match effect {
                                DialogueEffect::SetIntel(intel_id) => {
                                    intel_registry.set_intel(intel_id);
                                    intel_events.write(IntelDiscoveredEvent { intel_id: intel_id.clone() });
                                    info!("Intel discovered: {}", intel_id);
                                },
                                DialogueEffect::ChangeRelationship(from, to, relationship) => {
                                    info!("Relationship changed: {} -> {} = {:?}", from, to, relationship);
                                    // This would be handled by a relationship system
                                }
                            }
                        }
                        
                        // Update to the next dialogue node
                        active_dialogue.current_node_id = selected_option.next_node_id.clone();
                    }
                }
            }
        }
    }
}

/// System to update dialogue UI based on current dialogue state
fn update_dialogue_ui(
    dialogue_query: Query<&ActiveDialogue>,
    dialogue_assets: Res<Assets<DialogueTree>>,
    intel_registry: Res<IntelRegistry>,
    patrons_query: Query<(&PatronComponent, &Name)>,
) {
    for active_dialogue in dialogue_query.iter() {
        if let Some(dialogue_tree) = dialogue_assets.get(&active_dialogue.tree_handle) {
            if let Some(current_node) = dialogue_tree.nodes.get(&active_dialogue.current_node_id) {
                // In a real implementation, this would update UI elements with the dialogue text
                // and available options, filtering options based on conditions
                
                // For now, just print the current dialogue text and options
                info!("Dialogue: {}", current_node.text);
                
                for (i, option) in current_node.options.iter().enumerate() {
                    let is_available = option.conditions.iter().all(|condition| {
                        match condition {
                            DialogueCondition::HasIntel(intel_id) => {
                                intel_registry.has_intel(intel_id)
                            },
                            DialogueCondition::PatronMood(patron_name, expected_mood) => {
                                // Find the patron with the matching name and check their mood
                                for (patron, name) in patrons_query.iter() {
                                    if name.as_str() == patron_name || patron.name == *patron_name {
                                        // Convert the node::Mood to patron::Mood for comparison
                                        let patron_expected_mood = match expected_mood {
                                            nodes::Mood::Neutral => PatronMood::Neutral,
                                            nodes::Mood::Calm => PatronMood::Calm,
                                            nodes::Mood::Aggressive => PatronMood::Aggressive,
                                            nodes::Mood::Truthful => PatronMood::Truthful,
                                            nodes::Mood::Energized => PatronMood::Energized,
                                            nodes::Mood::Glitched => PatronMood::Glitched,
                                        };
                                        return patron.current_mood == patron_expected_mood;
                                    }
                                }
                                // If patron not found, condition fails
                                false
                            }
                        }
                    });
                    
                    if is_available {
                        info!("Option {}: {}", i, option.text);
                    }
                }
            }
        }
    }
} 
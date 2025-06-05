use bevy::prelude::*;
use std::collections::HashMap;

/// Component for tracking a patron's mood and personality state
#[derive(Component, Debug, Clone)]
pub struct Patron {
    pub name: String,
    pub current_mood: Mood,
    pub base_personality: Personality,
    pub current_dialogue_node: String,
}

/// Different mood states that patrons can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mood {
    Neutral,
    Calm,
    Aggressive,
    Truthful,
    Energized,
    Glitched,
    // Additional moods as needed
}

impl Default for Mood {
    fn default() -> Self {
        Self::Neutral
    }
}

/// Base personalities for each patron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Personality {
    Secretive,  // Zara
    Volatile,   // Kael
    Artificial, // Unit 734
    Creative,   // Lyra
}

/// Resource to track relationships between patrons
#[derive(Resource, Debug, Default)]
pub struct RelationshipRegistry {
    // from_patron -> to_patron -> relationship_type
    relationships: HashMap<String, HashMap<String, Relationship>>,
}

/// Relationship types between patrons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relationship {
    Friendly,
    Neutral,
    Suspicious,
    Hostile,
}

impl Default for Relationship {
    fn default() -> Self {
        Self::Neutral
    }
}

impl RelationshipRegistry {
    /// Get the relationship between two patrons
    pub fn get_relationship(&self, from: &str, to: &str) -> Relationship {
        self.relationships
            .get(from)
            .and_then(|relations| relations.get(to))
            .copied()
            .unwrap_or(Relationship::Neutral)
    }

    /// Set the relationship between two patrons
    pub fn set_relationship(&mut self, from: &str, to: &str, relationship: Relationship) {
        self.relationships
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string(), relationship);
    }
}

/// Plugin for patron-related systems
pub struct PatronPlugin;

impl Plugin for PatronPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RelationshipRegistry>()
           .add_systems(Startup, setup_initial_patrons);
    }
}

/// System to handle patron mood changes from drink effects
pub fn apply_mood_effect(
    entity: Entity, 
    mood: Mood,
) -> impl FnMut(Query<&mut Patron>) + Send + Sync + 'static {
    move |mut patron_query: Query<&mut Patron>| {
        if let Ok(mut patron) = patron_query.get_mut(entity) {
            patron.current_mood = mood;
        }
    }
}

/// System to spawn the four main patrons of the game
fn setup_initial_patrons(mut commands: Commands) {
    // Zara - The secretive security officer
    commands.spawn((
        Patron {
            name: "Zara".to_string(),
            current_mood: Mood::Neutral,
            base_personality: Personality::Secretive,
            current_dialogue_node: "zara_intro".to_string(),
        },
        Name::new("Zara"),
    ));
    
    // Kael - The volatile mercenary
    commands.spawn((
        Patron {
            name: "Kael".to_string(),
            current_mood: Mood::Aggressive,
            base_personality: Personality::Volatile,
            current_dialogue_node: "kael_intro".to_string(),
        },
        Name::new("Kael"),
    ));
    
    // Unit 734 - The artificial intelligence
    commands.spawn((
        Patron {
            name: "Unit 734".to_string(),
            current_mood: Mood::Neutral,
            base_personality: Personality::Artificial,
            current_dialogue_node: "unit_intro".to_string(),
        },
        Name::new("Unit 734"),
    ));
    
    // Lyra - The creative artist
    commands.spawn((
        Patron {
            name: "Lyra".to_string(),
            current_mood: Mood::Calm,
            base_personality: Personality::Creative,
            current_dialogue_node: "lyra_intro".to_string(),
        },
        Name::new("Lyra"),
    ));
} 
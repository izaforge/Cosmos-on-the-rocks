use super::nodes::DialogueTree;
use bevy::prelude::*;
use std::collections::HashMap;

/// Component for tracking a patron's personality state
#[derive(Component, Debug, Clone)]
pub struct Patron {
    pub name: String,
    pub base_personality: Personality,
    pub current_dialogue_node: String,
    pub dialogue_asset: Option<Handle<DialogueTree>>, // Added to store dialogue handle
}

/// Component for tracking a patron's happiness level (0-100)
#[derive(Component, Debug, Clone)]
pub struct Happiness {
    pub value: u8,
}

impl Default for Happiness {
    fn default() -> Self {
        Self { value: 50 }
    }
}

/// Component for tracking a patron's sadness level (0-100)
#[derive(Component, Debug, Clone)]
pub struct Sadness {
    pub value: u8,
}

impl Default for Sadness {
    fn default() -> Self {
        Self { value: 20 }
    }
}

/// Component for tracking a patron's anger level (0-100)
#[derive(Component, Debug, Clone)]
pub struct Anger {
    pub value: u8,
}

impl Default for Anger {
    fn default() -> Self {
        Self { value: 30 }
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
        info!("PatronPlugin initialized!");
        info!("This plugin will spawn the four main patrons: Zara, Kael, Unit 734, and Lyra");

        app.init_resource::<RelationshipRegistry>()
            .add_systems(Startup, setup_initial_patrons)
            .add_systems(Update, load_zara_dialogue.run_if(run_once)); // Only run once
    }
}

/// Condition to run a system only once
fn run_once() -> bool {
    static mut HAS_RUN: bool = false;
    unsafe {
        if !HAS_RUN {
            HAS_RUN = true;
            true
        } else {
            false
        }
    }
}

/// System to handle patron emotion changes from drink effects
pub fn apply_emotion_effect(
    entity: Entity,
    happiness_delta: i16,
    sadness_delta: i16,
    anger_delta: i16,
) -> impl FnMut(Query<(&mut Happiness, &mut Sadness, &mut Anger)>) + Send + Sync + 'static {
    move |mut emotion_query: Query<(&mut Happiness, &mut Sadness, &mut Anger)>| {
        if let Ok((mut happiness, mut sadness, mut anger)) = emotion_query.get_mut(entity) {
            // Apply changes with clamping between 0-100
            happiness.value = (happiness.value as i16 + happiness_delta).clamp(0, 100) as u8;
            sadness.value = (sadness.value as i16 + sadness_delta).clamp(0, 100) as u8;
            anger.value = (anger.value as i16 + anger_delta).clamp(0, 100) as u8;
        }
    }
}

/// System to spawn the four main patrons of the game
fn setup_initial_patrons(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("============================================");
    info!("Setting up initial patrons");

    // Zara - The secretive security officer
    // Load Zara's dialogue from assets/dialogues/zara.yarn
    let zara_dialogue = asset_server.load("dialogues/zara.yarn");

    let zara_entity = commands
        .spawn((
            Patron {
                name: "Zara".to_string(),
                base_personality: Personality::Secretive,
                current_dialogue_node: "zara_intro".to_string(),
                dialogue_asset: Some(zara_dialogue), // Store the dialogue handle
            },
            Happiness { value: 50 },
            Sadness { value: 20 },
            Anger { value: 30 },
            Name::new("Zara"),
        ))
        .id();
    info!("Spawned Zara with entity ID: {:?}", zara_entity);

    info!("Initial patrons setup complete");
    info!("============================================");
}

/// System to check if Zara's dialogue is properly loaded
fn load_zara_dialogue(
    patrons: Query<(Entity, &Patron)>,
    dialogue_assets: Res<Assets<DialogueTree>>,
) {
    for (_entity, patron) in patrons.iter() {
        if patron.name == "Zara" {
            if let Some(dialogue_handle) = &patron.dialogue_asset {
                if let Some(dialogue_tree) = dialogue_assets.get(dialogue_handle) {
                    // Print more visible debug info
                    println!("============================================");
                    println!("✅ SUCCESS: ZARA DIALOGUE LOADED FROM zara.dialogue.ron");
                    println!("Number of dialogue nodes: {}", dialogue_tree.nodes.len());
                    println!("Starting node: {}", dialogue_tree.starting_node);

                    if let Some(intro_node) = dialogue_tree.nodes.get("zara_intro") {
                        println!("Zara intro text: {}", intro_node.text);
                        println!("Available options: {}", intro_node.options.len());
                        for (i, option) in intro_node.options.iter().enumerate() {
                            println!("  Option {}: {}", i + 1, option.text);
                        }
                    } else {
                        println!("❌ ERROR: 'zara_intro' node not found in dialogue tree!");
                    }
                    println!("============================================");
                } else {
                    println!("❌ ERROR: Failed to get dialogue tree asset for Zara!");
                }
            } else {
                println!("❌ ERROR: No dialogue asset handle for Zara!");
            }
        }
    }
}

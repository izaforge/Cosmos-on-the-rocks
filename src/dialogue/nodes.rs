use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in a dialogue tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
}

/// Represents a selectable option in a dialogue node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    pub text: String,
    pub next_node_id: String,
    pub conditions: Vec<DialogueCondition>,
    pub effects: Vec<DialogueEffect>,
}

/// Types of emotions that can be checked or modified
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmotionType {
    Happiness,
    Sadness,
    Anger,
}

/// Comparison operators for emotion checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    Equal,
}

/// Conditions that must be met for a dialogue option to be available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueCondition {
    HasIntel(String),         // e.g., HasIntel("Intel_Zara_Secret")
    EmotionCheck(String, EmotionType, u8, Comparison), // e.g., EmotionCheck("Zara", EmotionType::Happiness, 70, Comparison::GreaterThan)
}

/// Effects that occur when a dialogue option is selected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueEffect {
    SetIntel(String),         // e.g., SetIntel("Intel_Zara_Secret")
    ChangeRelationship(String, String, Relationship), // e.g., ChangeRelationship("Zara", "Kael", Relationship::Hostile)
    ModifyEmotion(String, EmotionType, i16), // e.g., ModifyEmotion("Zara", EmotionType::Happiness, 15)
}

/// Base personalities for each patron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Personality {
    Secretive,  // Zara
    Volatile,   // Kael
    Artificial, // Unit 734
    Creative,   // Lyra
}

/// Relationship types between patrons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Relationship {
    Friendly,
    Neutral,
    Suspicious,
    Hostile,
}

/// Complete dialogue tree for a patron or scenario
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct DialogueTree {
    pub starting_node: String,
    pub nodes: HashMap<String, DialogueNode>,
}

/// Dialogue asset loader for Bevy
#[derive(Default)]
pub struct DialogueAssetLoader;

impl AssetLoader for DialogueAssetLoader {
    type Asset = DialogueTree;
    type Settings = ();
    type Error = ron::Error;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl std::future::Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            let dialogue_tree = ron::de::from_bytes::<DialogueTree>(&bytes)?;
            Ok(dialogue_tree)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["dialogue.ron"]
    }
}

/// Plugin to register dialogue assets
pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DialogueTree>()
           .init_asset_loader::<DialogueAssetLoader>();
    }
} 
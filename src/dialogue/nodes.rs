use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use std::collections::HashMap;

/// Represents a node in a dialogue tree
#[derive(Debug, Clone)]
pub struct DialogueNode {
    pub id: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
}

/// Represents a selectable option in a dialogue node
#[derive(Debug, Clone)]
pub struct DialogueOption {
    pub text: String,
    pub next_node_id: String,
    pub conditions: Vec<DialogueCondition>,
    pub effects: Vec<DialogueEffect>,
}

/// Types of emotions that can be checked or modified
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmotionType {
    Happiness,
    Sadness,
    Anger,
}

/// Comparison operators for emotion checks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    Equal,
}

/// Conditions that must be met for a dialogue option to be available
#[derive(Debug, Clone)]
pub enum DialogueCondition {
    HasIntel(String), // e.g., HasIntel("Intel_Zara_Secret")
    EmotionCheck(String, EmotionType, u8, Comparison), // e.g., EmotionCheck("Zara", EmotionType::Happiness, 70, Comparison::GreaterThan)
}

/// Effects that occur when a dialogue option is selected
#[derive(Debug, Clone)]
pub enum DialogueEffect {
    SetIntel(String), // e.g., SetIntel("Intel_Zara_Secret")
    ChangeRelationship(String, String, Relationship), // e.g., ChangeRelationship("Zara", "Kael", Relationship::Hostile)
    ModifyEmotion(String, EmotionType, i16), // e.g., ModifyEmotion("Zara", EmotionType::Happiness, 15)
}

/// Base personalities for each patron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Personality {
    Secretive,  // Zara
    Volatile,   // Kael
    Artificial, // Unit 734
    Creative,   // Coda
}

/// Relationship types between patrons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relationship {
    Friendly,
    Neutral,
    Suspicious,
    Hostile,
}

/// Complete dialogue tree for a patron or scenario
#[derive(Debug, Clone, Asset, TypePath)]
pub struct DialogueTree {
    pub starting_node: String,
    pub nodes: HashMap<String, DialogueNode>,
}

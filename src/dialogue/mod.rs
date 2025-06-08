pub mod intel;
mod nodes;
pub mod patrons;
mod systems;

// Use more specific exports to avoid conflicts
pub use intel::{IntelDiscoveredEvent, IntelPlugin, IntelRegistry};
pub use nodes::{
    Comparison, DialogueCondition, DialogueEffect, DialogueNode, DialogueOption, DialogueTree,
    EmotionType,
};
pub use patrons::{Anger, Happiness, Patron, PatronPlugin, RelationshipRegistry, Sadness};
pub use systems::{
    ActiveDialogue, DialogueInteractionPlugin, DialogueOptionButton, DialogueOptionSelected,
};

// Re-export the enums with specific namespaces to avoid conflicts
pub mod node_types {
    pub use super::nodes::{Comparison, EmotionType, Personality, Relationship};
}

pub mod patron_types {
    pub use super::patrons::{Personality, Relationship};
}

mod nodes;
pub mod intel;
mod systems;
pub mod patrons;

// Use more specific exports to avoid conflicts
pub use nodes::{DialogueNode, DialogueOption, DialogueCondition, DialogueEffect, DialogueTree, DialoguePlugin as NodesPlugin, EmotionType, Comparison};
pub use intel::{IntelRegistry, IntelDiscoveredEvent, IntelPlugin};
pub use systems::{DialogueInteractionPlugin, ActiveDialogue, DialogueOptionButton, DialogueOptionSelected};
pub use patrons::{Patron, RelationshipRegistry, PatronPlugin, Happiness, Sadness, Anger};

// Re-export the enums with specific namespaces to avoid conflicts
pub mod node_types {
    pub use super::nodes::{Personality, Relationship, EmotionType, Comparison};
}

pub mod patron_types {
    pub use super::patrons::{Personality, Relationship};
} 
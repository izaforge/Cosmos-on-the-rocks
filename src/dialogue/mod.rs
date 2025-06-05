mod nodes;
pub mod intel;
mod systems;
mod patrons;

// Use more specific exports to avoid conflicts
pub use nodes::{DialogueNode, DialogueOption, DialogueCondition, DialogueEffect, DialogueTree, DialoguePlugin as NodesPlugin};
pub use intel::{IntelRegistry, IntelDiscoveredEvent, IntelPlugin};
pub use systems::{DialogueInteractionPlugin, ActiveDialogue, DialogueOptionButton, DialogueOptionSelected};
pub use patrons::{Patron, RelationshipRegistry, PatronPlugin};

// Re-export the enums with specific namespaces to avoid conflicts
pub mod node_types {
    pub use super::nodes::{Mood, Personality, Relationship};
}

pub mod patron_types {
    pub use super::patrons::{Mood, Personality, Relationship};
} 
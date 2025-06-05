use bevy::prelude::*;
use std::collections::HashMap;

/// Resource to track discovered information and intel flags
#[derive(Resource)]
pub struct IntelRegistry {
    flags: HashMap<String, bool>,
}

impl Default for IntelRegistry {
    fn default() -> Self {
        Self {
            flags: HashMap::new(),
        }
    }
}

impl IntelRegistry {
    /// Check if a specific intel flag is set
    pub fn has_intel(&self, intel_id: &str) -> bool {
        *self.flags.get(intel_id).unwrap_or(&false)
    }

    /// Set an intel flag to true
    pub fn set_intel(&mut self, intel_id: impl Into<String>) {
        self.flags.insert(intel_id.into(), true);
    }

    /// Clear an intel flag (set to false)
    pub fn clear_intel(&mut self, intel_id: &str) {
        self.flags.insert(intel_id.to_string(), false);
    }

    /// Get all currently active intel flags
    pub fn get_active_intel(&self) -> Vec<String> {
        self.flags
            .iter()
            .filter_map(|(id, active)| if *active { Some(id.clone()) } else { None })
            .collect()
    }
}

/// Events emitted when intel is discovered or changed
#[derive(Event)]
pub struct IntelDiscoveredEvent {
    pub intel_id: String,
}

/// Systems to handle intel-related logic
pub struct IntelPlugin;

impl Plugin for IntelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IntelRegistry>()
           .add_event::<IntelDiscoveredEvent>()
           .add_systems(Update, update_intel_from_dialogue);
    }
}

/// System to update the intel registry based on dialogue outcomes
fn update_intel_from_dialogue(
    _intel_registry: ResMut<IntelRegistry>,
    _intel_events: EventWriter<IntelDiscoveredEvent>,
    // Additional queries for dialogue interaction would go here
) {
    // This system will be expanded as more of the dialogue system is implemented
    // It will listen for dialogue selections and update the intel registry accordingly
} 
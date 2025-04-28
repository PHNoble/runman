use bevy::prelude::*;

// Component for ownership/faction information
#[derive(Component, Debug, Clone)]
pub struct Ownership {
    pub faction: FactionId,
    pub team: TeamId,
    pub controller_type: ControllerType,
}

// Faction identification 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactionId {
    Player(u32),    // Specific player number
    Neutral,        // Neutral/passive units
    Creep,          // Hostile non-player units
    Environment,    // Map elements
}

// Team grouping (alliances)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamId {
    Team(u32),      // Specific team number
    Neutral,        // No team affiliation
    FFA,            // Free-for-all (all hostile)
}

// Controller type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerType {
    Human,          // Controlled by human player
    AI,             // Controlled by AI
    Automatic,      // Controlled by map logic
}

impl Default for Ownership {
    fn default() -> Self {
        Self {
            faction: FactionId::Neutral,
            team: TeamId::Neutral,
            controller_type: ControllerType::Automatic,
        }
    }
}
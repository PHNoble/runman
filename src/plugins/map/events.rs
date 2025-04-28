use bevy::prelude::*;
use super::grid::{GridCoord, TerrainType};
use crate::components::faction::FactionId;

/// Event for when a map is loaded
#[derive(Event)]
pub struct MapLoadedEvent {
    pub map_name: String,
    pub width: i32,
    pub height: i32,
}

/// Event for when terrain is modified
#[derive(Event)]
pub struct TerrainModifiedEvent {
    pub coord: GridCoord,
    pub new_terrain: TerrainType,
    pub timestamp: f64,
}

/// Event for when a unit moves to a grid position
#[derive(Event)]
pub struct UnitMoveEvent {
    pub entity: Entity,
    pub from: GridCoord,
    pub to: GridCoord,
    pub timestamp: f64,
}

/// Event for when a building is placed
#[derive(Event)]
pub struct BuildingPlacedEvent {
    pub entity: Entity,
    pub faction: FactionId,
    pub position: GridCoord,
    pub size: (i32, i32), // width, height in grid cells
    pub timestamp: f64,
}

/// Event for when terrain is revealed (fog of war)
#[derive(Event)]
pub struct TerrainRevealedEvent {
    pub center: GridCoord,
    pub radius: i32,
    pub faction: FactionId,
    pub timestamp: f64,
}

// Event for pathfinding requests
#[derive(Event)]
pub struct PathfindingRequestEvent {
    pub entity: Entity,
    pub from: GridCoord, 
    pub to: GridCoord,
    pub timestamp: f64,
}

// Event for pathfinding results
#[derive(Event)]
pub struct PathfindingResultEvent {
    pub entity: Entity,
    pub path: Vec<GridCoord>,
    pub success: bool,
    pub timestamp: f64,
}
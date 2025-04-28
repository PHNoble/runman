use bevy::prelude::*;

/// Grid coordinates for map locations (separate from world Transform)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: i32,
    pub y: i32,
}

/// Defines terrain types for each grid cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    Grass,
    Dirt,
    Stone,
    Water,
    Forest,
    Mountain,
}

/// Properties of individual grid cells
#[derive(Component, Debug, Clone)]
pub struct GridCell {
    pub terrain: TerrainType,
    pub walkable: bool,
    pub buildable: bool,
    pub elevation: f32,
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            terrain: TerrainType::Grass,
            walkable: true,
            buildable: true,
            elevation: 0.0,
        }
    }
}

/// Resource that defines the map grid configuration
#[derive(Resource)]
pub struct MapGrid {
    pub width: i32,
    pub height: i32,
    pub cell_size: f32,
    /// Maps grid coordinates to entity IDs containing the cell data
    cells: HashMap<GridCoord, Entity>,
}

impl MapGrid {
    /// Create a new map grid with specified dimensions
    pub fn new(width: i32, height: i32, cell_size: f32) -> Self {
        Self {
            width,
            height,
            cell_size,
            cells: HashMap::new(),
        }
    }

    /// Register a cell entity with its grid coordinates
    pub fn register_cell(&mut self, coord: GridCoord, entity: Entity) {
        self.cells.insert(coord, entity);
    }
    
    /// Get the entity at the specified grid coordinates
    pub fn get_cell_entity(&self, coord: GridCoord) -> Option<&Entity> {
        self.cells.get(&coord)
    }

    /// Convert world position to grid coordinates
    pub fn world_to_grid(&self, world_pos: Vec3) -> GridCoord {
        // In RTS games, typically using X and Z as the ground plane
        GridCoord {
            x: (world_pos.x / self.cell_size).floor() as i32,
            y: (world_pos.z / self.cell_size).floor() as i32,
        }
    }
    
    /// Convert grid coordinates to world position (centered in cell)
    pub fn grid_to_world(&self, coord: GridCoord, elevation: f32) -> Vec3 {
        Vec3::new(
            (coord.x as f32 + 0.5) * self.cell_size,
            elevation, // Y is up in Bevy's coordinate system
            (coord.y as f32 + 0.5) * self.cell_size,
        )
    }
    
    /// Check if coordinates are within map bounds
    pub fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
}

use std::collections::HashMap;
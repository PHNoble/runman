use bevy::prelude::*;
use super::{
    grid::{GridCoord, GridCell, TerrainType, MapGrid},
    events::MapLoadedEvent,
    LoadedMap,
};
use crate::plugins::camera::CameraGround;

/// System for loading a map from a file or specific configuration
pub fn load_map(
    map_name: &str,
    width: i32,
    height: i32,
    cell_size: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    map_loaded_events: &mut EventWriter<MapLoadedEvent>,
    loaded_map: &mut ResMut<LoadedMap>,
) {
    // Clear existing map entities
    // In a real implementation, you'd clean up the existing map entities first
    
    // Create ground plane mesh
    let mesh = meshes.add(
        Plane3d::default()
            .mesh()
            .size(cell_size * width as f32, cell_size * height as f32)
            .subdivisions(width as u32)
    );
    
    // Basic green material for ground
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.3),
        perceptual_roughness: 0.9,
        ..default()
    });
    
    // Create main terrain entity with camera ground component
    commands.spawn((
        // Core components (Mesh3d requires Material3d)
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(
            (width as f32 * cell_size) / 2.0 - (cell_size / 2.0),
            0.0,
            (height as f32 * cell_size) / 2.0 - (cell_size / 2.0),
        ),
        super::TerrainMesh,
        CameraGround,
    ));
    
    // Create grid resource
    commands.insert_resource(MapGrid::new(width, height, cell_size));
    
    // In a real implementation, you'd load the map data from a file here
    // For now, we'll create a simple procedural map
    
    // Generate grid cells
    for x in 0..width {
        for y in 0..height {
            let coord = GridCoord { x, y };
            
            // Create some procedural terrain features
            let terrain = match (x, y) {
                // Water around the edges
                (0..=3, _) | (_, 0..=3) => TerrainType::Water,
                (x_val, _) if x_val >= width - 4 => TerrainType::Water,
                (_, y_val) if y_val >= height - 4 => TerrainType::Water,
                
                // Some mountains
                (x, y) if (x as f32 - width as f32 / 3.0).powi(2) + 
                          (y as f32 - height as f32 / 3.0).powi(2) < 10.0 => TerrainType::Mountain,
                
                // Some forests
                (x, y) if (x + y) % 7 == 0 => TerrainType::Forest,
                
                // Default to grass
                _ => TerrainType::Grass,
            };
            
            let mut cell = GridCell::default();
            cell.terrain = terrain;
            
            // Set properties based on terrain type
            match terrain {
                TerrainType::Water => {
                    cell.walkable = false;
                    cell.buildable = false;
                },
                TerrainType::Mountain => {
                    cell.walkable = false;
                    cell.buildable = false;
                    cell.elevation = 2.0;
                },
                TerrainType::Forest => {
                    cell.buildable = false;
                },
                _ => {
                    // Default already set
                }
            }
            
            // Spawn the cell entity
            commands.spawn((coord, cell));
        }
    }
    
    // Send map loaded event
    map_loaded_events.write(MapLoadedEvent {
        map_name: map_name.to_string(),
        width,
        height,
    });
    
    // Update loaded map resource
    loaded_map.name = map_name.to_string();
    loaded_map.loaded = true;
}

/// Command to load a specific map
#[derive(Event)]
pub struct LoadMapCommand {
    pub map_name: String,
}

/// System to handle load map commands
pub fn handle_load_map_commands(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_loaded_events: EventWriter<MapLoadedEvent>,
    mut loaded_map: ResMut<LoadedMap>,
    mut load_events: EventReader<LoadMapCommand>,
) {
    for event in load_events.read() {
        // In a real implementation, you'd load different maps based on the name
        // For now, we just have one map configuration
        load_map(
            &event.map_name,
            64, // Larger map
            64,
            1.0,
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut map_loaded_events,
            &mut loaded_map,
        );
    }
}

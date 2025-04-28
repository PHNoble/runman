use bevy::prelude::*;
use crate::plugins::camera::CameraGround;

mod grid;
mod events;
mod loader;
mod unit_examples;

pub use grid::{GridCoord, GridCell, TerrainType, MapGrid};
pub use events::*;
pub use loader::LoadMapCommand;

/// Marker component for the terrain mesh
#[derive(Component)]
pub struct TerrainMesh;

/// Main map plugin
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register events
            .add_event::<MapLoadedEvent>()
            .add_event::<TerrainModifiedEvent>()
            .add_event::<UnitMoveEvent>()
            .add_event::<BuildingPlacedEvent>()
            .add_event::<TerrainRevealedEvent>()
            .add_event::<PathfindingRequestEvent>()
            .add_event::<PathfindingResultEvent>()
            .add_event::<LoadMapCommand>()
            
            // Register resources
            .init_resource::<LoadedMap>()
            
            // Register systems
            .add_systems(Startup, initialize_default_map)
            .add_systems(Update, (
                handle_terrain_modification,
                handle_pathfinding_requests,
                loader::handle_load_map_commands,
            ))
            // Register the grid cells after map initialization
            .add_systems(PostStartup, register_grid_cells)
            // Spawn example units after the map is loaded
            .add_systems(PostStartup, unit_examples::spawn_example_units);
    }
}

/// Resource to track currently loaded map
#[derive(Resource, Default)]
pub struct LoadedMap {
    pub name: String,
    pub loaded: bool,
}

/// Initialize a default map for testing
fn initialize_default_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_loaded_events: EventWriter<MapLoadedEvent>,
    mut loaded_map: ResMut<LoadedMap>,
) {
    // Map dimensions
    let width = 32 as i32;
    let height = 32;
    let cell_size = 1.0;
    
    // Create ground plane mesh
    let mesh = meshes.add(
            Plane3d::default()
            .mesh()
            .size(cell_size * width as f32, cell_size * width as f32)
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
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(
                (width as f32 * cell_size) / 2.0 - (cell_size / 2.0),
                0.0,
                (height as f32 * cell_size) / 2.0 - (cell_size / 2.0),
        ),
        TerrainMesh,
        CameraGround,
    ));
    
    // Create grid resource
    commands.insert_resource(MapGrid::new(width, height, cell_size));
    
    // Generate grid cells
    for x in 0..width {
        for y in 0..height {
            let coord = GridCoord { x, y };
            let cell = GridCell::default();
            
            // Spawn minimal entity for each cell
            commands.spawn((
                coord,
                cell,
            ));
            
            // The MapGrid will be inserted as a resource first, so we'll register cells in a follow-up system
            // For now we just create the entities
        }
    }
    
    // Send map loaded event
    map_loaded_events.write(MapLoadedEvent {
        map_name: "default".to_string(),
        width,
        height,
    });
    
    // Update loaded map resource
    loaded_map.name = "default".to_string();
    loaded_map.loaded = true;
}

/// Handle terrain modification events
fn handle_terrain_modification(
    mut events: EventReader<TerrainModifiedEvent>,
    mut grid_cells: Query<&mut GridCell>,
    grid_coords: Query<(Entity, &GridCoord)>,
) {
    for event in events.read() {
        // Find the entity with the matching grid coordinate
        for (entity, coord) in grid_coords.iter() {
            if *coord == event.coord {
                // Update the cell's terrain type
                if let Ok(mut cell) = grid_cells.get_mut(entity) {
                    cell.terrain = event.new_terrain;
                    
                    // Additional logic for terrain changes
                    match event.new_terrain {
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
                            // Default for other terrain types
                            cell.walkable = true;
                            cell.buildable = true;
                        }
                    }
                }
                break;
            }
        }
    }
}

/// Handle pathfinding requests
fn handle_pathfinding_requests(
    mut request_events: EventReader<PathfindingRequestEvent>,
    mut result_events: EventWriter<PathfindingResultEvent>,
    grid: Res<MapGrid>,
    _grid_cells: Query<&GridCell>,
    _grid_coords: Query<(Entity, &GridCoord)>,
    time: Res<Time>,
) {
    for event in request_events.read() {
        // Basic pathfinding implementation
        // For now, just return a direct path if possible
        // In the future, implement A* or other pathfinding algorithms
        
        if !grid.in_bounds(event.from) || !grid.in_bounds(event.to) {
            // Invalid coordinates
            result_events.write(PathfindingResultEvent {
                entity: event.entity,
                path: Vec::new(),
                success: false,
                timestamp: time.elapsed_secs_f64(),
            });
            continue;
        }
        
        // TODO: Implement actual pathfinding
        // For now, just return a direct path
        let path = vec![event.from, event.to];
        
        result_events.write(PathfindingResultEvent {
            entity: event.entity,
            path,
            success: true,
            timestamp: time.elapsed_secs_f64(),
        });
    }
}

/// System to register cell entities with grid (runs once after initialization)
pub fn register_grid_cells(
    mut grid: ResMut<MapGrid>,
    grid_entities: Query<(Entity, &GridCoord)>,
) {
    for (entity, coord) in grid_entities.iter() {
        grid.register_cell(*coord, entity);
    }
}

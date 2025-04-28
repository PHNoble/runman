use bevy::prelude::*;
use crate::components::unit::{Unit, UnitType, UnitState, Statsheet};
use crate::components::faction::{Ownership, FactionId, TeamId, ControllerType};
use super::grid::{GridCoord, MapGrid};

/// Component to mark visualized unit entities
#[derive(Component)]
pub struct UnitVisualization;

/// Spawn a simple test unit at a specified grid coordinate
pub fn spawn_test_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    grid: &Res<MapGrid>,
    coord: GridCoord,
) -> Entity {
    // Create unit stats
    let mut stats = Statsheet::default();
    stats.strength = 20.0;
    stats.agility = 15.0;
    stats.intelligence = 10.0;
    stats.calculate_derived_stats();
    stats.initialize();
    
    // Create unit ownership
    let ownership = Ownership {
        faction: FactionId::Player(1),
        team: TeamId::Team(1),
        controller_type: ControllerType::Human,
    };
    
    // Create a simple mesh for the unit
    let mesh = meshes.add(Mesh::from(Capsule3d {
        radius: 0.3,
        half_length: 0.8,
        ..default()
    }));
    
    // Create a blue material for the unit
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.2, 0.8),
        ..default()
    });
    
    // Get the world position from grid coordinates
    let position = grid.grid_to_world(coord, 0.4); // Slight elevation
    
    // Spawn the unit entity
    commands.spawn((
        // Core unit components
        Unit {
            name: "Test Warrior".to_string(),
            unit_type: UnitType::Melee,
            state: UnitState::Idle,
            target: None,
        },
        stats,
        ownership,
        
        // Visual representation
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        UnitVisualization,
    )).id()
}

/// System that spawns some test units on the map
pub fn spawn_example_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid: Res<MapGrid>,
    query: Query<&UnitVisualization>,
) {
    // Only spawn units if none exist yet
    if query.is_empty() {
        // Spawn several units in different locations
        spawn_test_unit(
            &mut commands,
            &mut meshes,
            &mut materials,
            &grid, 
            GridCoord { x: 10, y: 10 }
        );
        
        spawn_test_unit(
            &mut commands,
            &mut meshes,
            &mut materials,
            &grid, 
            GridCoord { x: 12, y: 10 }
        );
        
        spawn_test_unit(
            &mut commands,
            &mut meshes,
            &mut materials,
            &grid, 
            GridCoord { x: 10, y: 12 }
        );
        
        // Spawn a "target" visualization at the center
        let target_mesh = meshes.add(Mesh::from(Sphere {
            radius: 0.2,
            ..default()
        }));
        
        let target_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.2, 0.2),
            ..default()
        });
        
        let center_pos = grid.grid_to_world(GridCoord { x: 15, y: 15 }, 0.2);
        
        commands.spawn((
                Mesh3d(target_mesh),
                MeshMaterial3d(target_material),
                Transform::from_translation(center_pos),
            Name::new("Target"),
        ));
    }
}

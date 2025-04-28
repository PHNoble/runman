use bevy::prelude::*;

/// System to set up basic scene lighting
pub fn setup_lighting(mut commands: Commands) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.6, 0.6, 0.6),
        brightness: 0.4,
        affects_lightmapped_meshes: true,
    });

    // Directional light - simulates sunlight
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.85),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0)
            .looking_at(Vec3::new(-0.5, -1.0, -0.5), Vec3::Y),
    ));
}


use crate::prelude::*;

// TODO: made redundant by asset loading
const LEVEL_WIDTH: i32 = 20;
const LEVEL_HEIGHT: i32 = 20;
const LEVEL_SIZE: f32 = 20.0;
const LEVEL_GROUND_Y: f32 = -LEVEL_SIZE / 2.0;

/// Marker component for the level entity
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct Level;

/// Spawns a very simple level; no input at this moment
pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .id();

    // Ground
    for x in 0..LEVEL_WIDTH {
        for z in 0..LEVEL_HEIGHT {
            // TODO: If we have a single elevation, we can just have a giant plane
            let ground = commands
                .spawn((
                    Name::new("Ground"),
                    position_to_transform(x, LEVEL_GROUND_Y, z),
                    Visibility::default(),
                    Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(LEVEL_SIZE / 2.0)))),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
                ))
                .id();

            commands.entity(level).add_child(ground);
        }
    }

    // Walls
    for z in [-1, LEVEL_HEIGHT] {
        for x in 0..LEVEL_WIDTH {
            let wall = commands
                .spawn((
                    Name::new("Wall"),
                    position_to_transform(x, 0.0, z),
                    Visibility::default(),
                    Mesh3d(meshes.add(Cuboid::from_length(LEVEL_SIZE))),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::BLACK))),
                ))
                .id();

            commands.entity(level).add_child(wall);
        }
    }
    for x in [-1, LEVEL_WIDTH] {
        for z in 0..LEVEL_HEIGHT {
            let wall = commands
                .spawn((
                    Name::new("Wall"),
                    position_to_transform(x, 0.0, z),
                    Visibility::default(),
                    Mesh3d(meshes.add(Cuboid::from_length(LEVEL_SIZE))),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::BLACK))),
                ))
                .id();

            commands.entity(level).add_child(wall);
        }
    }

    commands.spawn((
        Name::new("Plane"),
        Transform::default(),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        Mesh3d(meshes.add(Plane3d::new(Vec3::X, Vec2::splat(10.)))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::BLACK))),
    ));

    commands.spawn((
        Name::new("Cube"),
        Transform::from_xyz(0., 0., -20.),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        Mesh3d(meshes.add(Cuboid::new(3., 3., 3.))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(
            bevy::color::palettes::css::BLUE,
        ))),
    ));
}

fn position_to_transform(x: i32, y: f32, z: i32) -> Transform {
    // TODO: Offset by some amount for now, because I dont want to move camera :D
    Transform::from_xyz((x - 1) as f32 * LEVEL_SIZE, y, (z - 1) as f32 * LEVEL_SIZE)
}

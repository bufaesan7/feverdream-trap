use crate::chunk::Chunk;
use crate::prelude::*;

// number of chunks per axis
const GRID_SIZE: usize = 5;
const CHUNKS: usize = GRID_SIZE * GRID_SIZE;
const TILE_SIZE: f32 = 20.;

// Marker component for the level entity
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct Level;

/// spawn demo level with a grid of entities grouped in chunks
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

    let mut chunk_index = 0;

    for chunk_x in 0..GRID_SIZE {
        for chunk_z in 0..GRID_SIZE {
            let transform = grid_position_to_transform(chunk_x, chunk_z);
            let chunk = commands
                .spawn((Visibility::default(), Chunk, transform, ChildOf(level)))
                .id();
            let chunk_color = Hsva::hsv((chunk_index as f32 / CHUNKS as f32) * 360., 1.0, 1.0);

            info!(
                "spawned chunk at {} with index {}",
                transform.translation.xz(),
                chunk_index
            );

            // spawn debug cube of chunk color
            if chunk_index == 8 {
                commands.spawn((
                    Name::new("Cube"),
                    Transform::from_xyz(0., 2., 0.),
                    Visibility::Visible,
                    DespawnOnExit(Screen::Gameplay),
                    RigidBody::Static,
                    Collider::cuboid(2., 2., 2.),
                    Mesh3d(meshes.add(Cuboid::new(2., 2., 2.))),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(chunk_color))),
                    ChildOf(chunk),
                ));
            }

            if chunk_index == 17 {
                commands.spawn((
                    Name::new("Sphere"),
                    Transform::from_xyz(0., 2., 0.),
                    Visibility::Visible,
                    DespawnOnExit(Screen::Gameplay),
                    RigidBody::Static,
                    Collider::sphere(0.7),
                    Mesh3d(meshes.add(Sphere::new(0.7))),
                    MeshMaterial3d(materials.add(StandardMaterial::from_color(chunk_color))),
                    ChildOf(chunk),
                ));
            }

            // Ground
            commands.spawn((
                Name::new("Ground"),
                Transform::default(),
                Visibility::Visible,
                RigidBody::Static,
                Collider::cuboid(TILE_SIZE, 0.1, TILE_SIZE),
                Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(TILE_SIZE / 2.)))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(chunk_color))),
                ChildOf(chunk),
            ));

            chunk_index += 1;
        }
    }
}

fn grid_position_to_transform(x: usize, z: usize) -> Transform {
    Transform::from_xyz(x as f32 * TILE_SIZE, 0.0, z as f32 * TILE_SIZE)
}

use crate::chunk_assets::ChunkElementShape;
use crate::level::*;
use crate::physics::GameLayer;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_spawn_chunk)
        .add_observer(on_despawn_chunk);
}

pub const CHUNK_SIZE: f32 = 5.;

#[derive(Default, Component, Reflect)]
#[require(Transform, Visibility, Sensor, LevelComponent)]
#[reflect(Component)]
pub struct Chunk;

#[derive(Default, Component, Reflect, Debug, Copy, Clone)]
#[require(Chunk)]
#[reflect(Component)]
pub struct ChunkId(pub u32);

#[derive(Debug, Event)]
pub struct SpawnChunk {
    pub level: Entity,
    pub id: ChunkId,
    pub grid_position: Vec2,
    pub descriptor: Handle<ChunkDescriptor>,
    #[cfg(feature = "dev")]
    pub show_wireframe: bool,
}

#[derive(Debug, Event)]
pub struct DespawnChunk(pub ChunkId);

/// TODO move to game once chunk_asset handle this two components embedding
#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct SwapSensorChunk(pub ChunkId, pub ChunkId);

#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct ReplaceAssetSensorChunk(pub ChunkId, pub Handle<ChunkDescriptor>);

pub fn on_spawn_chunk(
    event: On<SpawnChunk>,
    mut commands: Commands,
    descriptors: Res<Assets<ChunkDescriptor>>,
    elements: Res<Assets<ChunkElement>>,
) {
    let level = event.level;
    let id = event.id;
    let grid_position = event.grid_position;

    let descriptor = descriptors.get(&event.descriptor).unwrap();
    let elements = descriptor
        .elements
        .iter()
        .filter_map(|e| elements.get(&e.0));

    let transform = Transform::from_xyz(
        grid_position.x * CHUNK_SIZE,
        0.0,
        grid_position.y * CHUNK_SIZE,
    );

    let chunk_entity = commands
        .spawn((
            Name::new(format!("Chunk ({}, {})", grid_position.x, grid_position.y)),
            Visibility::default(),
            Chunk,
            id,
            transform,
            LevelCollider::Cube { length: CHUNK_SIZE },
            RigidBody::Static,
            Sensor,
            CollisionEventsEnabled,
            CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player]),
            ChildOf(level),
        ))
        .id();

    let color = Color::srgb(0.95, 0.95, 0.95);

    for element in elements {
        let mut element_entity = commands.spawn((
            Name::new(element.name.clone()),
            element.transform,
            Visibility::Visible,
            ChildOf(chunk_entity),
        ));

        match &element.shape {
            ChunkElementShape::Plane => {
                element_entity.insert(LevelComponent3d::Plane {
                    size: Vec2::splat(0.5),
                    color,
                });
            }
            ChunkElementShape::Cube => {
                element_entity.insert(LevelComponent3d::Cube { length: 1., color });
            }
            ChunkElementShape::Sphere => {
                element_entity.insert(LevelComponent3d::Sphere { radius: 1., color });
            }
            ChunkElementShape::Gltf { mesh_path, .. } => {
                element_entity.insert(LevelComponentGltf {
                    path: mesh_path.clone(),
                });
            }
        };
    }

    #[cfg(feature = "dev")]
    // Show chunk wireframe
    if event.show_wireframe {
        commands.spawn((
            Name::new("Wireframe"),
            Collider::cuboid(CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE),
            DebugRender::none().with_collider_color(Color::srgb(1., 0., 0.)),
        ));
    }
}

pub fn on_despawn_chunk(
    event: On<DespawnChunk>,
    mut commands: Commands,
    chunks: Query<(Entity, &ChunkId), With<Chunk>>,
) {
    let ChunkId(chunk_id) = event.0;

    for (entity, ChunkId(id)) in &chunks {
        if chunk_id == *id {
            commands.entity(entity).despawn();
            return;
        }
    }
}

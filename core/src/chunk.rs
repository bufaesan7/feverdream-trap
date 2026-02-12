use crate::chunk_assets::{ChunkElement, ChunkElementShape};
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
    pub elements: Vec<ChunkElement>,
}

#[derive(Debug, Event)]
pub struct DespawnChunk(pub ChunkId);

/// TODO move to game once chunk_asset handel this two components embeding
#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct SwapSensorChunk(pub ChunkId, pub ChunkId);

#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct ReplaceAssetSensorChunk(pub ChunkId, pub String);

pub fn on_spawn_chunk(event: On<SpawnChunk>, mut commands: Commands) {
    let level = event.level;
    let id = event.id;
    let grid_position = event.grid_position;
    let elements = &event.elements;

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
        let level_component = match &element.shape {
            ChunkElementShape::Plane => LevelComponent3d::Plane {
                size: Vec2::splat(0.5),
                color,
            },
            ChunkElementShape::Cube => LevelComponent3d::Cube { length: 1., color },
            ChunkElementShape::Sphere => LevelComponent3d::Sphere { radius: 1., color },
            s => panic!("Shape is not supported yet {:?}", s),
        };

        commands.spawn((
            Name::new(element.name.clone()),
            element.transform,
            Visibility::Visible,
            level_component,
            ChildOf(chunk_entity),
        ));
    }

    // TODO embed in chunk_asset
    if id.0 == 23 {
        commands
            .entity(chunk_entity)
            .insert(ReplaceAssetSensorChunk(
                ChunkId(9),
                "chunks/demo/floor_only".to_string(),
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
            info!("Chunk {chunk_id} has been despawned");
            return;
        }
    }
}

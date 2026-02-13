use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;

use crate::chunk_assets::{ChunkDescriptorAsset, ChunkElementShape, ChunkMarker};
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
    pub components: Vec<ChunkMarker>,
}

#[derive(Debug, Event)]
pub struct DespawnChunk(pub ChunkId);

#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct SwapSensorChunk(pub ChunkId, pub ChunkId);

#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct ReplaceAssetSensorChunk(pub ChunkId, pub Handle<ChunkDescriptor>);

#[cfg(feature = "dev_native")]
pub static CHUNK_WIREFRAMES_ENABLED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
#[component(on_add)]
pub struct ChunkMarkers(pub Vec<ChunkMarker>);

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SpawnMarker(pub Transform);

impl ChunkMarkers {
    fn on_add(mut world: DeferredWorld<'_>, hook: HookContext) {
        let markers = world.get::<ChunkMarkers>(hook.entity).unwrap().0.clone();

        for marker in markers {
            match marker {
                ChunkMarker::PlayerSpawn(t) => {
                    world.commands().entity(hook.entity).insert(SpawnMarker(t));
                }
                ChunkMarker::SwapSensor(a, b) => {
                    world
                        .commands()
                        .entity(hook.entity)
                        .insert(SwapSensorChunk(ChunkId(a), ChunkId(b)));
                }
                ChunkMarker::ReplaceAssetSensor(id, descriptor_name) => {
                    let path = ChunkDescriptorAsset::path_from_name(&descriptor_name);
                    let handle: Handle<ChunkDescriptor> =
                        world.load_asset(path.to_string_lossy().into_owned());

                    world
                        .commands()
                        .entity(hook.entity)
                        .insert(ReplaceAssetSensorChunk(ChunkId(id), handle));
                }
                _ => {}
            }
        }
    }
}

pub fn on_spawn_chunk(
    event: On<SpawnChunk>,
    mut commands: Commands,
    descriptors: Res<Assets<ChunkDescriptor>>,
    elements: Res<Assets<ChunkElement>>,
) {
    let level = event.level;
    let id = event.id;
    let grid_position = event.grid_position;

    let Some(descriptor) = descriptors.get(&event.descriptor) else {
        return;
    };
    let elements = descriptor
        .elements
        .iter()
        .filter_map(|e| elements.get(&e.0));

    let transform = Transform::from_xyz(
        grid_position.x * CHUNK_SIZE,
        0.0,
        grid_position.y * CHUNK_SIZE,
    );

    let mut chunk_cmds = commands.spawn((
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
    ));
    #[cfg(feature = "dev_native")]
    if CHUNK_WIREFRAMES_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
        chunk_cmds.insert(DebugRender::none().with_collider_color(Color::srgb(1., 0., 0.)));
    }

    if !event.components.is_empty() {
        chunk_cmds.insert(ChunkMarkers(event.components.clone()));
    }

    let chunk_entity = chunk_cmds.id();

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

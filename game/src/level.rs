use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

// number of chunks per axis
const GRID_SIZE: i32 = 5;
const CHUNKS: i32 = GRID_SIZE * GRID_SIZE;
const TILE_SIZE: f32 = 20.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_chunk_transform);
}

/// This is the public interface for a position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Position {
    x: i32,
    z: i32,
}

// This is an internal component, it is automatically handled when chunks are inserted on a Level
#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
#[require(Chunk)]
struct ChunkPosition(Position);

/// This should contain arbitrary date: e.g. list of tile entities, a model or some other asset
#[derive(Default, Component)]
#[require(Transform, Visibility)]
#[component(on_remove = Self::on_remove)]
pub struct Chunk;

impl Chunk {
    // If chunk is removed for some reason, detach from level
    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        // TODO: Does not work if the whole entity is removed. Any way to handle this?
        /*
        let chunk = ctx.entity;

        if let Some(childof) = world.entity(chunk).get::<ChildOf>() {
            let level = childof.parent();

            world.commands().entity(level).detach_chunk(chunk);
        }
        */
    }
}

/// Component containing all chunks, with their respective chunks
/// Chunks can only be added by using the EntityCommands extension
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Level {
    chunks: HashMap<Position, Entity>,
}

impl Level {
    pub fn get(&self, position: &Position) -> Option<Entity> {
        self.chunks.get(position).copied()
    }
}

/// This handles adding chunks to levels
/// Adds a ChunkPosition component to the chunk entity
/// Updates the level component
/// Adds a parent/child relationship
pub trait LevelChunkEntityCommandsExt<'a> {
    fn insert_chunk(&mut self, chunk: Entity, position: Position) -> &mut EntityCommands<'a>;
    fn detach_chunk(&mut self, chunk: Entity) -> &mut EntityCommands<'a>;
    fn detach_chunk_by_position(&mut self, position: Position) -> &mut EntityCommands<'a>;
}

impl<'a> LevelChunkEntityCommandsExt<'a> for EntityCommands<'a> {
    fn insert_chunk(&mut self, chunk: Entity, position: Position) -> &mut EntityCommands<'a> {
        self.queue(move |mut entity: EntityWorldMut| {
            entity.insert_chunk(chunk, position);
        })
    }

    fn detach_chunk(&mut self, chunk: Entity) -> &mut EntityCommands<'a> {
        self.queue(move |mut entity: EntityWorldMut| {
            entity.detach_chunk(chunk);
        })
    }

    fn detach_chunk_by_position(&mut self, position: Position) -> &mut EntityCommands<'a> {
        self.queue(move |mut entity: EntityWorldMut| {
            entity.detach_chunk_by_position(position);
        })
    }
}

/// This handles adding chunks to levels
/// Adds a ChunkPosition component to the chunk entity
/// Updates the level component
/// Adds a parent/child relationship
pub trait LevelChunkEntityWorldMutExt<'w> {
    fn insert_chunk(&mut self, chunk: Entity, position: Position) -> &mut EntityWorldMut<'w>;
    fn detach_chunk(&mut self, chunk: Entity) -> &mut EntityWorldMut<'w>;
    fn detach_chunk_by_position(&mut self, position: Position) -> &mut EntityWorldMut<'w>;
}

impl<'w> LevelChunkEntityWorldMutExt<'w> for EntityWorldMut<'w> {
    fn insert_chunk(&mut self, chunk: Entity, position: Position) -> &mut EntityWorldMut<'w> {
        let mut has_level = false;
        if let Some(mut level) = self.get_mut::<Level>() {
            has_level = true;

            // Update Level component
            let replaced_chunk = level.chunks.insert(position, chunk);

            // Handle overwriting existing entries
            if let Some(replaced_chunk) = replaced_chunk {
                self.detach_chunk(replaced_chunk);
            }

            self.add_child(chunk);
        }

        if has_level {
            // Add ChunkPosition component to Entity
            self.world_scope(|world: &mut World| {
                world.entity_mut(chunk).insert(ChunkPosition(position));
            });
        }

        self
    }

    fn detach_chunk(&mut self, chunk: Entity) -> &mut EntityWorldMut<'w> {
        // Get the position and detach by position
        if let Some(chunk_position) = self.world().entity(chunk).get::<ChunkPosition>() {
            self.detach_chunk_by_position(chunk_position.0);
        }

        self
    }

    // TODO: Might be able to simplify to remove Chunk, if on_remove_chunk handles all the rest
    fn detach_chunk_by_position(&mut self, position: Position) -> &mut EntityWorldMut<'w> {
        if let Some(mut level) = self.get_mut::<Level>() {
            // Remove chunk from level
            let removed_chunk = level.chunks.remove(&position);

            if let Some(removed_chunk) = removed_chunk {
                // Remove Parent/Child relationship
                self.detach_child(removed_chunk);
                // Remove the ChunkPosition component from chunk
                self.world_scope(|world: &mut World| {
                    world.entity_mut(removed_chunk).remove::<ChunkPosition>();
                });
            }
        }

        self
    }
}

// Whenever ChunkPosition changes
fn update_chunk_transform(
    mut chunks: Query<(&mut Transform, &ChunkPosition), Changed<ChunkPosition>>,
) {
    for (mut transform, chunk_position) in &mut chunks {
        *transform = grid_position_to_transform(chunk_position.0.x, chunk_position.0.z);
    }
}

/// spawn demo level with a grid of entities grouped in chunks
pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Level::default(),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .id();

    let mut chunk_index = 0;

    for chunk_x in 0..GRID_SIZE {
        for chunk_z in 0..GRID_SIZE {
            let position = Position {
                x: chunk_x,
                z: chunk_z,
            };
            let chunk = commands
                .spawn((
                    Visibility::default(),
                    Chunk,
                    ChunkPosition(position),
                    ChildOf(level),
                ))
                .id();
            commands.entity(level).insert_chunk(chunk, position);
            let chunk_color = Hsva::hsv((chunk_index as f32 / CHUNKS as f32) * 360., 1.0, 1.0);

            info!("spawned chunk at {:?} with index {}", position, chunk_index);

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

fn grid_position_to_transform(x: i32, z: i32) -> Transform {
    Transform::from_xyz(x as f32 * TILE_SIZE, 0.0, z as f32 * TILE_SIZE)
}

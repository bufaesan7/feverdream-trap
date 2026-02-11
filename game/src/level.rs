use bevy::{
    color::palettes::css::YELLOW,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    pbr::ExtendedMaterial,
};

use crate::{
    interaction::{
        DebugInteraction, HighlightExtension, HighlightStorageBuffer, Interact, Interactable,
    },
    prelude::*,
};

use crate::character_controller::GameLayer;
use crate::chunk::{Chunk, ChunkId, SwapSensorChunk, SwappableChunk};

// number of chunks per axis
const GRID_SIZE: usize = 5;
const CHUNKS: usize = GRID_SIZE * GRID_SIZE;
const TILE_SIZE: f32 = 20.;

// Marker component for the level entity
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(LevelComponent)]
pub struct Level;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(DespawnOnExit<Screen> = DespawnOnExit(Screen::Gameplay))]
/// Marker component for each [`Entity`] that is part of the level scene
pub struct LevelComponent;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(LevelComponent)]
#[component(on_add)]
/// Abstract [`Mesh3d`] and [`MeshMaterial3d`] insertion to avoid inserting them in the
/// [`DynamicScene`] storage.
pub enum LevelComponent3d {
    Plane {
        size: Vec2,
        color: Color,
    },
    Cube {
        length: f32,
        color: Color,
        rigid_body: RigidBody,
    },
    Sphere {
        radius: f32,
        color: Color,
    },
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(LevelComponent)]
#[component(on_add)]
/// Abstract [`Mesh3d`] and [`MeshMaterial3d`] insertion to avoid inserting them in the
/// [`DynamicScene`] storage.
pub enum LevelCollider {
    Cube { length: f32 },
}

impl LevelCollider {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        if !world.contains_resource::<Assets<Mesh>>()
            || !world.contains_resource::<Assets<StandardMaterial>>()
        {
            // Skip this hook when we're constructing a [`DynamicScene`]
            return;
        }

        let collider_type = world.get::<LevelCollider>(hook.entity).unwrap().clone();

        let collider = match collider_type {
            LevelCollider::Cube { length, .. } => Collider::cuboid(length, length, length),
            // LevelCollider::Sphere { radius, .. } => Collider::sphere(radius),
        };

        let mut commands = world.commands();

        commands.entity(hook.entity).insert((collider,));
    }
}

impl LevelComponent3d {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        if !world.contains_resource::<Assets<Mesh>>()
            || !world.contains_resource::<Assets<ExtendedMaterial<StandardMaterial, HighlightExtension>>>()
        {
            // Skip this hook when we're constructing a [`DynamicScene`]
            return;
        }

        let mesh_type = world.get::<LevelComponent3d>(hook.entity).unwrap().clone();

        let mut meshes: Mut<Assets<Mesh>> = world.resource_mut();
        let mesh = match mesh_type {
            LevelComponent3d::Plane { size, .. } => meshes.add(Plane3d::new(Vec3::Y, size)),
            LevelComponent3d::Cube { length, .. } => meshes.add(Cuboid::from_length(length)),
            LevelComponent3d::Sphere { radius, .. } => meshes.add(Sphere::new(radius)),
        };

        let colors = world.resource::<HighlightStorageBuffer>().0.clone();
        let mut materials: Mut<Assets<ExtendedMaterial<StandardMaterial, HighlightExtension>>> =
            world.resource_mut();
        let material = match mesh_type {
            LevelComponent3d::Plane { .. } => materials.add(ExtendedMaterial {
                base: StandardMaterial::from_color(Color::WHITE),
                extension: HighlightExtension { colors },
            }),
            LevelComponent3d::Cube { color, .. } => materials.add(ExtendedMaterial {
                base: StandardMaterial::from_color(color),
                extension: HighlightExtension { colors },
            }),
            LevelComponent3d::Sphere { color, .. } => materials.add(ExtendedMaterial {
                base: StandardMaterial::from_color(color),
                extension: HighlightExtension { colors },
            }),
        };

        let collider = match mesh_type {
            LevelComponent3d::Plane { size, .. } => Collider::cuboid(size.x * 2., 0.1, size.y * 2.),
            LevelComponent3d::Cube { length, .. } => Collider::cuboid(length, length, length),
            LevelComponent3d::Sphere { radius, .. } => Collider::sphere(radius),
        };

        let rigid_body = match mesh_type {
            LevelComponent3d::Plane { .. } => RigidBody::Static,
            LevelComponent3d::Cube { rigid_body, .. } => rigid_body,
            LevelComponent3d::Sphere { .. } => RigidBody::Static,
        };

        let mut commands = world.commands();

        commands.entity(hook.entity).insert((
            rigid_body,
            collider,
            Mesh3d(mesh),
            MeshMaterial3d(material),
        ));
    }
}

/// spawn demo level with a grid of chunked entities
pub fn spawn_level(mut commands: Commands) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    let mut chunk_index = 0;

    for chunk_x in 0..GRID_SIZE {
        for chunk_z in 0..GRID_SIZE {
            let transform = grid_position_to_transform(chunk_x, chunk_z);

            let chunk = commands
                .spawn((
                    Name::new("Chunk"),
                    Visibility::default(),
                    Chunk,
                    ChunkId(chunk_index as u32),
                    transform,
                    LevelCollider::Cube { length: TILE_SIZE },
                    RigidBody::Static,
                    Sensor,
                    CollisionEventsEnabled,
                    CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player]),
                    ChildOf(level),
                ))
                .id();

            // swap the two special chunks with cube and sphere when the player enters the chunk with id 1
            if chunk_index == 12 {
                commands
                    .entity(chunk)
                    .insert(SwapSensorChunk(ChunkId(17), ChunkId(8)));
            }

            let chunk_color: Color =
                Hsva::hsv((chunk_index as f32 / CHUNKS as f32) * 360., 1.0, 1.0).into();

            info!(
                "spawned chunk at {} with index {}",
                transform.translation.xz(),
                chunk_index
            );

            // Spawn debug entities
            if chunk_index == 1 {
                commands.entity(chunk).insert(SwappableChunk);
                commands.spawn((
                    Name::new("Cube"),
                    Transform::from_xyz(0., 1., 0.),
                    Visibility::Visible,
                    LevelComponent3d::Cube {
                        length: 2.,
                        color: chunk_color,
                        rigid_body: RigidBody::Dynamic,
                    },
                    ChildOf(chunk),
                ));
            }
            if chunk_index == 8 {
                commands.entity(chunk).insert(SwappableChunk);
                commands
                    .spawn((
                        Name::new("Cube"),
                        Transform::from_xyz(0., 1., 0.),
                        Visibility::Visible,
                        LevelComponent3d::Cube {
                            length: 2.,
                            color: chunk_color,
                            rigid_body: RigidBody::Dynamic,
                        },
                        Interactable,
                        ChildOf(chunk),
                    ))
                    .observe(|on_interact: On<Interact>| {
                        info!("Interact with {:?}", on_interact.entity);
                    }); // TODO: This does not work
            }
            if chunk_index == 17 {
                commands.entity(chunk).insert(SwappableChunk);
                commands.spawn((
                    Name::new("Sphere"),
                    Transform::from_xyz(0., 1.5, 0.),
                    Visibility::Visible,
                    LevelComponent3d::Sphere {
                        radius: 1.5,
                        color: chunk_color,
                    },
                    ChildOf(chunk),
                ));
            }

            // Ground
            commands.spawn((
                Name::new("Ground"),
                Transform::default(),
                Visibility::Visible,
                LevelComponent3d::Plane {
                    size: Vec2::splat(TILE_SIZE / 2.),
                    color: chunk_color,
                },
                ChildOf(chunk),
            ));

            // Walls
            let wall_height = 5.;
            let wall_thickness = 0.5;
            if chunk_x == 0 {
                // West wall
                commands.spawn((
                    Name::new("West Wall"),
                    Transform::from_xyz(-TILE_SIZE / 2., wall_height / 2., 0.)
                        .with_scale(Vec3::new(wall_thickness, wall_height, TILE_SIZE)),
                    Visibility::Visible,
                    LevelComponent3d::Cube {
                        length: 1.,
                        color: chunk_color,
                        rigid_body: RigidBody::Static,
                    },
                    ChildOf(chunk),
                ));
            }
            if chunk_x == GRID_SIZE - 1 {
                // East wall
                commands.spawn((
                    Name::new("East Wall"),
                    Transform::from_xyz(TILE_SIZE / 2., wall_height / 2., 0.)
                        .with_scale(Vec3::new(wall_thickness, wall_height, TILE_SIZE)),
                    Visibility::Visible,
                    LevelComponent3d::Cube {
                        length: 1.,
                        color: chunk_color,
                        rigid_body: RigidBody::Static,
                    },
                    ChildOf(chunk),
                ));
            }
            if chunk_z == 0 {
                // North wall
                commands.spawn((
                    Name::new("North Wall"),
                    Transform::from_xyz(0., wall_height / 2., -TILE_SIZE / 2.)
                        .with_scale(Vec3::new(TILE_SIZE, wall_height, wall_thickness)),
                    Visibility::Visible,
                    LevelComponent3d::Cube {
                        length: 1.,
                        color: chunk_color,
                        rigid_body: RigidBody::Static,
                    },
                    ChildOf(chunk),
                ));
            }
            if chunk_z == GRID_SIZE - 1 {
                // South wall
                commands.spawn((
                    Name::new("South Wall"),
                    Transform::from_xyz(0., wall_height / 2., TILE_SIZE / 2.)
                        .with_scale(Vec3::new(TILE_SIZE, wall_height, wall_thickness)),
                    Visibility::Visible,
                    LevelComponent3d::Cube {
                        length: 1.,
                        color: chunk_color,
                        rigid_body: RigidBody::Dynamic,
                    },
                ));
            }

            chunk_index += 1;
        }
    }

    commands.spawn((
        Name::new("Cube"),
        Transform::from_xyz(0., 30., 0.),
        Visibility::Visible,
        LevelComponent3d::Cube {
            length: 2.,
            color: YELLOW.into(),
            rigid_body: RigidBody::Dynamic,
        },
        Interactable,
        DebugInteraction,
    ));
}

fn grid_position_to_transform(x: usize, z: usize) -> Transform {
    Transform::from_xyz(x as f32 * TILE_SIZE, 0.0, z as f32 * TILE_SIZE)
}

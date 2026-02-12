use crate::chunk::{ChunkId, SpawnChunk};
use crate::chunk_assets::{ChunkDescriptor, ChunkElement, ChunkLayout};
use crate::prelude::*;
use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(LevelComponent)]
pub struct Level;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct LevelComponent;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(LevelComponent)]
#[component(on_add)]
pub struct LevelComponentGltf {
    pub path: String,
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(LevelComponent)]
#[component(on_add)]
/// Abstract [`Mesh3d`] and [`MeshMaterial3d`] insertion to avoid inserting them in the
/// [`DynamicScene`] storage.
pub enum LevelComponent3d {
    Plane { size: Vec2, color: Color },
    Cube { length: f32, color: Color },
    Sphere { radius: f32, color: Color },
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

impl LevelComponentGltf {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        if !world.contains_resource::<Assets<Mesh>>()
            || !world.contains_resource::<Assets<StandardMaterial>>()
        {
            // Skip this hook when we're constructing a [`DynamicScene`]
            return;
        }

        let component = world
            .get::<LevelComponentGltf>(hook.entity)
            .unwrap()
            .clone();
        let gltf_handle: Handle<Gltf> = world.load_asset(component.path);

        let scene = {
            let gltf_assets = world.resource::<Assets<Gltf>>();
            let Some(gltf) = gltf_assets.get(&gltf_handle) else {
                return;
            };
            gltf.scenes.first().cloned()
        };

        if let Some(scene) = scene {
            world.commands().entity(hook.entity).insert((
                SceneRoot(scene),
                DebugInteraction,
                Collider::cuboid(20.0, 20.0, 20.0),
            ));
        }
    }
}

impl LevelComponent3d {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        if !world.contains_resource::<Assets<Mesh>>()
            || !world.contains_resource::<Assets<StandardMaterial>>()
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

        let mut materials: Mut<Assets<StandardMaterial>> = world.resource_mut();

        let material = match mesh_type {
            LevelComponent3d::Plane { color, .. } => {
                materials.add(StandardMaterial::from_color(color))
            }
            LevelComponent3d::Cube { color, .. } => {
                materials.add(StandardMaterial::from_color(color))
            }
            LevelComponent3d::Sphere { color, .. } => {
                materials.add(StandardMaterial::from_color(color))
            }
        };

        let collider = match mesh_type {
            LevelComponent3d::Plane { size, .. } => Collider::cuboid(size.x * 2., 0.1, size.y * 2.),
            LevelComponent3d::Cube { length, .. } => Collider::cuboid(length, length, length),
            LevelComponent3d::Sphere { radius, .. } => Collider::sphere(radius),
        };

        let mut commands = world.commands();

        info!(
            "spawning component 3d components for entity {}",
            hook.entity
        );

        commands.entity(hook.entity).insert((
            RigidBody::Static,
            collider,
            Mesh3d(mesh),
            MeshMaterial3d(material),
        ));
    }
}

pub fn spawn_level_from_layout(
    mut commands: Commands,
    chunk_layout_storage: Res<ChunkLayoutStorage>,
    chunk_layouts: Res<Assets<ChunkLayout>>,
    chunk_descriptors: Res<Assets<ChunkDescriptor>>,
    chunk_elements: Res<Assets<ChunkElement>>,
) {
    let Some(layout) = chunk_layouts.get(&chunk_layout_storage.handle) else {
        warn!("Chunk layout not loaded yet");
        return;
    };

    // Compute grid bounds to support arbitrary layout coordinates (including negatives)
    let (min_z, max_z) = {
        let mut min_z = i32::MAX;
        let mut max_z = i32::MIN;
        for (&(_x, z), _) in &layout.grid {
            if z < min_z {
                min_z = z;
            }
            if z > max_z {
                max_z = z;
            }
        }
        (min_z, max_z)
    };

    let grid_size_z = max_z - min_z + 1;

    let level = commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    for ((x, z), descriptor_handle) in &layout.grid {
        let Some(chunk_descriptor) = chunk_descriptors.get(descriptor_handle) else {
            return;
        };

        let chunk_id = (*z + *x * grid_size_z) as u32;

        let elements = chunk_descriptor
            .elements
            .iter()
            .map(|element| chunk_elements.get(&element.0).unwrap().clone())
            .collect();

        commands.trigger(SpawnChunk {
            level,
            id: ChunkId(chunk_id),
            grid_position: Vec2::new(*x as f32, *z as f32),
            elements,
        });
    }
}

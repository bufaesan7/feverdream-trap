use crate::chunk::{ChunkId, SpawnChunk};
use crate::chunk_assets::ChunkLayout;
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
pub struct LevelComponent3d {
    pub shape: LevelComponentShape,
    pub color: Color,
    pub has_collider: bool,
}

#[derive(Reflect, Debug, Clone)]
pub enum LevelComponentShape {
    Plane { size: Vec2 },
    Cube { length: f32 },
    Sphere { radius: f32 },
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
                Collider::cuboid(15.0, 15.0, 15.0),
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

        let lvl_comp = world.get::<LevelComponent3d>(hook.entity).unwrap().clone();

        let mut meshes: Mut<Assets<Mesh>> = world.resource_mut();

        let mesh = match lvl_comp.shape {
            LevelComponentShape::Plane { size, .. } => meshes.add(Plane3d::new(Vec3::Y, size)),
            LevelComponentShape::Cube { length, .. } => meshes.add(Cuboid::from_length(length)),
            LevelComponentShape::Sphere { radius, .. } => meshes.add(Sphere::new(radius)),
        };

        let mut materials: Mut<Assets<StandardMaterial>> = world.resource_mut();

        let material = materials.add(StandardMaterial::from_color(lvl_comp.color));

        let collider_maybe = lvl_comp.has_collider.then(|| match lvl_comp.shape {
            LevelComponentShape::Plane { size, .. } => {
                Collider::cuboid(size.x * 2., 0.1, size.y * 2.)
            }
            LevelComponentShape::Cube { length, .. } => Collider::cuboid(length, length, length),
            LevelComponentShape::Sphere { radius, .. } => Collider::sphere(radius),
        });

        let mut cmds = world.commands();
        let mut entity_cmds = cmds.entity(hook.entity);

        entity_cmds.insert((RigidBody::Static, Mesh3d(mesh), MeshMaterial3d(material)));

        if let Some(collider) = collider_maybe {
            entity_cmds.insert(collider);
        }
    }
}

pub fn spawn_level_from_layout(
    mut commands: Commands,
    chunk_stash: Res<ChunkAssetStash>,
    chunk_layouts: Res<Assets<ChunkLayout>>,
) {
    let layout = chunk_layouts.get(&chunk_stash.layout).unwrap();

    let level = commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    for (chunk_id, entry) in layout.chunks.iter().enumerate() {
        commands.trigger(SpawnChunk {
            level,
            id: ChunkId(chunk_id as u32),
            grid_position: Vec2::new(entry.grid_pos.0 as f32, entry.grid_pos.1 as f32),
            descriptor: entry.descriptor.clone(),
            components: entry.components.clone(),
        });
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Fuse(pub bool);

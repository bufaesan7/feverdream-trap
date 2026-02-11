use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshTag,
    pbr::{ExtendedMaterial, MaterialExtension},
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};

use crate::{camera_controller::CameraMarker, prelude::*};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_systems(Update, focusable_highlight)
        .init_resource::<HighlightStorageBuffer>()
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, HighlightExtension>,
        >::default());
}

/// Indicates whether an entity can be interacted with
/// Adds/removes picking over/out observers on the entity that add/remove [`Focusable`]
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_add, on_remove)]
#[require(MeshTag)]
pub struct Interactable {
    distance: f32,
    over_observer: Entity,
    out_observer: Entity,
}

impl Interactable {
    pub fn new(distance: f32) -> Self {
        Self {
            distance,
            over_observer: Entity::PLACEHOLDER,
            out_observer: Entity::PLACEHOLDER,
        }
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        // Add Observer
        let over_observer = world
            .commands()
            .spawn(Observer::new(Self::on_over).with_entity(ctx.entity))
            .id();
        let out_observer = world
            .commands()
            .spawn(Observer::new(Self::on_out).with_entity(ctx.entity))
            .id();

        if let Some(mut interactable) = world.entity_mut(ctx.entity).get_mut::<Self>() {
            interactable.over_observer = over_observer;
            interactable.out_observer = out_observer;
        }
    }

    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        // Remove observer
        let observer = world
            .entity(ctx.entity)
            .get::<Self>()
            .map(|interactable| (interactable.over_observer, interactable.out_observer));

        if let Some(observer) = observer {
            world.commands().entity(observer.0).try_despawn();
            world.commands().entity(observer.1).try_despawn();
        }
    }

    fn on_over(over: On<Pointer<Over>>, mut commands: Commands) {
        commands.entity(over.entity).insert(Focusable);
    }

    fn on_out(over: On<Pointer<Out>>, mut commands: Commands) {
        commands.entity(over.entity).try_remove::<Focusable>();
    }
}

// This indicates that an interactable entity is focusable (because we are looking at it)
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_remove)]
struct Focusable;

impl Focusable {
    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        // Remove FocusTarget
        world
            .commands()
            .entity(ctx.entity)
            .try_remove::<FocusTarget>();
    }
}

// Sets MeshTag based on distance
fn focusable_highlight(
    mut commands: Commands,
    query: Query<(Entity, &Interactable), With<Focusable>>,
    camera: Single<Entity, With<CameraMarker>>,
    transforms: Query<&GlobalTransform>,
) {
    for (interactable_entity, interactable) in &query {
        if let Ok([interactable_transform, camera_transform]) =
            transforms.get_many([interactable_entity, *camera])
        {
            let distance = camera_transform
                .translation()
                .distance(interactable_transform.translation());

            if distance < interactable.distance {
                commands.entity(interactable_entity).insert(FocusTarget);
            } else {
                commands
                    .entity(interactable_entity)
                    .try_remove::<FocusTarget>();
            }
        }
    }
}

const DEFAULT_MESH_TAG: u32 = 0;
const HIGHLIGHT_MESH_TAG: u32 = 1;

/// This indicates that an interactable entity is in focus
/// It is highlighted and can be interacted with
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_add, on_remove)]
pub struct FocusTarget;

impl FocusTarget {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        // Set MeshTag
        if let Some(mut mesh_tag) = world.entity_mut(ctx.entity).get_mut::<MeshTag>() {
            mesh_tag.0 = HIGHLIGHT_MESH_TAG;
        }
    }

    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        // Reset MeshTag
        if let Some(mut mesh_tag) = world.entity_mut(ctx.entity).get_mut::<MeshTag>() {
            mesh_tag.0 = DEFAULT_MESH_TAG;
        }
    }
}

const SHADER_ASSET_PATH: &str = "shaders/blend.wgsl";

/// This resource holds the common color storage buffer for highlighting
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct HighlightStorageBuffer(pub Handle<ShaderStorageBuffer>);

impl FromWorld for HighlightStorageBuffer {
    fn from_world(world: &mut World) -> Self {
        let mut buffers = world
            .get_resource_mut::<Assets<ShaderStorageBuffer>>()
            .unwrap();
        let color_data: Vec<[f32; 4]> = vec![[1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 0.0]];

        Self(buffers.add(ShaderStorageBuffer::from(color_data)))
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HighlightExtension {
    #[storage(100, read_only)]
    pub colors: Handle<ShaderStorageBuffer>,
}

impl MaterialExtension for HighlightExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

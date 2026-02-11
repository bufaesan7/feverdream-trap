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
        .add_systems(Update, (interactable_in_range, interact))
        .init_resource::<HighlightStorageBuffer>()
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, HighlightExtension>,
        >::default());
}

const INTERACTION_DISTANCE: f32 = 10.0;

/// Indicates whether an entity can be interacted with
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[require(MeshTag)]
pub struct Interactable;

fn interactable_in_range(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    interactables: Query<&Interactable>,
    camera: Single<&GlobalTransform, With<CameraMarker>>,
    focus_targets: Query<Entity, With<FocusTarget>>,
) {
    let transform = camera.compute_transform();
    let hit = spatial_query.cast_ray_predicate(
        transform.translation,
        transform.forward(),
        INTERACTION_DISTANCE,
        true,
        &SpatialQueryFilter::default(),
        &|entity| interactables.contains(entity),
    );

    // TODO: This is not optimal
    for entity in focus_targets {
        commands.entity(entity).try_remove::<FocusTarget>();
    }
    if let Some(first_hit) = hit {
        commands.entity(first_hit.entity).insert(FocusTarget);
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
        let color_data: Vec<[f32; 4]> = vec![[1.0, 1.0, 1.0, 1.0], [0.9, 0.9, 0.9, 0.9]];

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

fn interact(mouse: Res<ButtonInput<MouseButton>>, focus_targets: Query<Entity, With<FocusTarget>>) {
    for entity in &focus_targets {
        if mouse.just_pressed(MouseButton::Left) {
            info!("Interact with {:?}", entity);
        }
    }
}

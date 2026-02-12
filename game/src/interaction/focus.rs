use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshTag,
    pbr::{ExtendedMaterial, MaterialExtension},
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};

use crate::{
    camera_controller::CameraMarker,
    interaction::{INTERACTION_DISTANCE, Interactable},
    prelude::*,
};

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<HighlightStorageBuffer>()
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, HighlightExtension>,
        >::default())
        .add_systems(Startup, extended_material_required_components)
        .add_systems(
            Update,
            interactable_in_range.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        );
}

// SpatialQuery ray cast from camera for interactable entities
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
        &SpatialQueryFilter::from_mask(GameLayer::Interactable),
        &|_| true,
    );

    // TODO: This is not optimal
    for entity in focus_targets {
        commands.entity(entity).try_remove::<FocusTarget>();
    }
    if let Some(first_hit) = hit
        && interactables.contains(first_hit.entity)
    {
        commands.entity(first_hit.entity).try_insert(FocusTarget);
    }
}

const DEFAULT_MESH_TAG: u32 = 0;
const HIGHLIGHT_MESH_TAG: u32 = 1;

fn recursive_set_meshtag(world: &mut DeferredWorld, entity: Entity, value: u32) {
    if let Ok(mut entityref) = world.get_entity_mut(entity)
        && let Some(mut mesh_tag) = entityref.get_mut::<MeshTag>()
    {
        mesh_tag.0 = value;
    }

    let Ok(entityref) = world.get_entity(entity) else {
        return;
    };

    let Some(children) = entityref.get::<Children>() else {
        return;
    };
    let children: Vec<Entity> = children.iter().collect();

    for child in children {
        recursive_set_meshtag(world, child, value);
    }
}

/// This indicates that an interactable entity is in focus
/// It is highlighted and can be interacted with
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_add, on_remove)]
pub struct FocusTarget;

impl FocusTarget {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        // Set MeshTag
        recursive_set_meshtag(&mut world, ctx.entity, HIGHLIGHT_MESH_TAG);
    }

    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        // Reset MeshTag
        recursive_set_meshtag(&mut world, ctx.entity, DEFAULT_MESH_TAG);
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

fn extended_material_required_components(world: &mut World) {
    world.register_required_components::<MeshMaterial3d<ExtendedMaterial<StandardMaterial, HighlightExtension>>, MeshTag>();
}

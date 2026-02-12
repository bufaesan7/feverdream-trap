use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshTag,
    pbr::{ExtendedMaterial, MaterialExtension},
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};

use crate::{
    camera_controller::CameraMarker,
    character_controller::GameLayer,
    chunk::{ChunkId, SwapChunks},
    prelude::*,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_systems(
            Update,
            (interactable_in_range, interact)
                .run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        )
        .init_resource::<HighlightStorageBuffer>()
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, HighlightExtension>,
        >::default());
}

const INTERACTION_DISTANCE: f32 = 10.0;

/// Indicates whether an entity can be interacted with
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(MeshTag)]
#[component(on_add)]
pub struct Interactable;

impl Interactable {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world
            .commands()
            .entity(ctx.entity)
            .insert(CollisionLayers::new(
                GameLayer::Interactable,
                LayerMask::ALL,
            ));
    }
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

#[derive(Debug, EntityEvent, Reflect)]
#[reflect(Event)]
pub struct Interact {
    pub entity: Entity,
}

impl From<Entity> for Interact {
    fn from(entity: Entity) -> Self {
        Interact { entity }
    }
}

fn interact(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    focus_targets: Query<Entity, With<FocusTarget>>,
) {
    for entity in &focus_targets {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(entity).trigger(Interact::from);
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(Interactable)]
pub struct DebugInteraction;

impl DebugInteraction {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().spawn(
            Observer::new(|on_interact: On<Interact>| {
                info!("interact with {:?}", on_interact.entity);
            })
            .with_entity(ctx.entity),
        );
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(Interactable)]
pub struct DespawnInteraction;

impl DespawnInteraction {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().spawn(
            Observer::new(|on_interact: On<Interact>, mut commands: Commands| {
                let command = |entity: EntityWorldMut| {
                    entity.despawn();
                };
                commands.entity(on_interact.entity).queue_silenced(command);
            })
            .with_entity(ctx.entity),
        );
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(Interactable)]
pub struct ChunkSwapInteraction(pub ChunkId, pub ChunkId);

impl ChunkSwapInteraction {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().spawn(
            Observer::new(
                |on_interact: On<Interact>,
                 mut commands: Commands,
                 chunk_swap_interaction: Query<&Self>| {
                    if let Ok(ChunkSwapInteraction(chunk1, chunk2)) =
                        chunk_swap_interaction.get(on_interact.entity)
                    {
                        commands.trigger(SwapChunks(*chunk1, *chunk2));
                    }
                },
            )
            .with_entity(ctx.entity),
        );
    }
}

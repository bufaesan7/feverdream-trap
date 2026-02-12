use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshTag,
};

use crate::{
    camera_controller::CameraMarker,
    interaction::{INTERACTION_DISTANCE, Interactable},
    prelude::*,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
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

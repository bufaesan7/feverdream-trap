use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::{
    chunk::SwapChunks, interaction::{Interact, Interactable}, prelude::*
};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(Interactable)]
pub(super) struct DebugInteraction;

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
pub struct SwapChunksInteraction(pub ChunkId, pub ChunkId);

impl SwapChunksInteraction {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().spawn(
            Observer::new(
                |on_interact: On<Interact>,
                 mut commands: Commands,
                 chunk_swap_interaction: Query<&Self>| {
                    if let Ok(SwapChunksInteraction(chunk1, chunk2)) =
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

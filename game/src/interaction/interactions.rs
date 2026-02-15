use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::{
    chunk::SwapChunks,
    interaction::{Fuse, Interact, Interactable},
    prelude::*,
};

pub(super) fn register_required_components(world: &mut World) {
    world.register_required_components::<DebugInteraction, Interactable>();
    world.register_required_components::<DespawnInteraction, Interactable>();
    world.register_required_components::<SwapChunksInteraction, Interactable>();
    world.register_required_components::<PlaySoundEffectInteraction, Interactable>();
    world.register_required_components::<PickupFuseInteraction, Interactable>();
    world.register_required_components::<ElevatorInteraction, Interactable>();
}

pub(super) fn register_component_hooks(world: &mut World) {
    world.register_component_hooks::<DebugInteraction>().on_add(
        |mut world: DeferredWorld, ctx: HookContext| {
            world.commands().spawn(
                Observer::new(|on_interact: On<Interact>| {
                    info!("interact with {:?}", on_interact.entity);
                })
                .with_entity(ctx.entity),
            );
        },
    );

    world
        .register_component_hooks::<DespawnInteraction>()
        .on_add(|mut world: DeferredWorld, ctx: HookContext| {
            world.commands().spawn(
                Observer::new(|on_interact: On<Interact>, mut commands: Commands| {
                    let command = |entity: EntityWorldMut| {
                        entity.despawn();
                    };
                    commands.entity(on_interact.entity).queue_silenced(command);
                })
                .with_entity(ctx.entity),
            );
        });

    world
        .register_component_hooks::<SwapChunksInteraction>()
        .on_add(|mut world: DeferredWorld, ctx: HookContext| {
            world.commands().spawn(
                Observer::new(
                    |on_interact: On<Interact>,
                     mut commands: Commands,
                     chunk_swap_interaction: Query<&SwapChunksInteraction>| {
                        if let Ok(SwapChunksInteraction(chunk1, chunk2)) =
                            chunk_swap_interaction.get(on_interact.entity)
                        {
                            commands.trigger(SwapChunks(*chunk1, *chunk2));
                        }
                    },
                )
                .with_entity(ctx.entity),
            );
        });

    world
        .register_component_hooks::<PlaySoundEffectInteraction>()
        .on_add(|mut world: DeferredWorld, ctx: HookContext| {
            world.commands().spawn(
                Observer::new(
                    |on_interact: On<Interact>,
                     mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     play_sound_effect_interaction: Query<&PlaySoundEffectInteraction>| {
                        if let Ok(PlaySoundEffectInteraction(path)) = play_sound_effect_interaction.get(on_interact.entity) {
                            commands.spawn(sound_effect(asset_server.load(path)));
                        }
                    },
                )
                .with_entity(ctx.entity),
            );
        });

    world
        .register_component_hooks::<PickupFuseInteraction>()
        .on_add(|mut world: DeferredWorld, ctx: HookContext| {
            world.commands().spawn(
                Observer::new(|_on_interact: On<Interact>, mut fuse: ResMut<Fuse>| {
                    info!("Picked up fuse");
                    fuse.0 = true;
                })
                .with_entity(ctx.entity),
            );
        });

    world
        .register_component_hooks::<ElevatorInteraction>()
        .on_add(|mut world: DeferredWorld, ctx: HookContext| {
            world.commands().spawn(
                Observer::new(
                    |_on_interact: On<Interact>,
                     fuse: Res<Fuse>,
                     mut next_screen: ResMut<NextState<Screen>>| {
                        if fuse.0 {
                            info!("YES");
                            next_screen.set(Screen::GameOver);
                        } else {
                            info!("NO");
                        }
                    },
                )
                .with_entity(ctx.entity),
            );
        });
}

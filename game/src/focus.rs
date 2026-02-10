use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin);
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
#[component(on_add, on_remove)]
pub struct Focusable {
    hover_observer: Entity,
    unhover_observer: Entity,
}

impl Default for Focusable {
    fn default() -> Self {
        Self {
            hover_observer: Entity::PLACEHOLDER,
            unhover_observer: Entity::PLACEHOLDER,
        }
    }
}

impl Focusable {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let hover_observer = world
            .commands()
            .spawn(Observer::new(Self::on_over).with_entity(ctx.entity))
            .id();
        let unhover_observer = world
            .commands()
            .spawn(Observer::new(Self::on_out).with_entity(ctx.entity))
            .id();

        if let Some(mut focusable) = world.entity_mut(ctx.entity).get_mut::<Self>() {
            focusable.hover_observer = hover_observer;
            focusable.unhover_observer = unhover_observer;
        }
    }

    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        if let Some(focusable) = world.entity(ctx.entity).get::<Self>().copied() {
            world
                .commands()
                .entity(focusable.hover_observer)
                .try_despawn();
            world
                .commands()
                .entity(focusable.unhover_observer)
                .try_despawn();
        }
    }

    fn on_over(over: On<Pointer<Over>>) {
        // TODO: Highlight in some way
        info!("Over {}", over.entity);
    }

    fn on_out(out: On<Pointer<Out>>) {
        // TODO: Remove highlighting in some way
        info!("Out {}", out.entity);
    }
}

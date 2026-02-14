pub mod audio;
pub mod cursor;

use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct DespawnAfter(Timer);

impl DespawnAfter {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

pub fn despawn_after_internal(
    commands: &mut Commands,
    entity: Entity,
    despawn_after: &mut DespawnAfter,
    delta: Duration,
) {
    if despawn_after.0.tick(delta).just_finished() {
        commands.entity(entity).try_despawn();
    }
}

#[derive(Debug, Reflect, Clone, Copy)]
pub enum FadeMode {
    In,
    Out,
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
pub struct FadeText {
    mode: FadeMode,
    timer: Timer,
}

impl FadeText {
    pub fn new(mode: FadeMode, duration: Duration) -> Self {
        Self {
            mode,
            timer: Timer::new(duration, TimerMode::Once),
        }
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let Some(fade_text) = world.entity(ctx.entity).get::<Self>() else {
            return;
        };
        let mode = fade_text.mode;
        let duration = fade_text.timer.duration();

        match mode {
            FadeMode::In => { /* Nothing to do */ }
            FadeMode::Out => {
                /* Insert DespawnAfter component with the same duration */
                world
                    .commands()
                    .entity(ctx.entity)
                    .insert(DespawnAfter::new(duration));
            }
        }
    }
}

pub fn fade_text_internal(query: &mut Query<(&mut TextColor, &mut FadeText)>, delta: Duration) {
    for (mut color, mut fade) in query {
        fade.timer.tick(delta);

        let fraction = fade.timer.fraction();
        let fraction = match fade.mode {
            FadeMode::In => fraction,
            FadeMode::Out => 1.0 - fraction,
        };

        color.0.set_alpha(fraction);
    }
}

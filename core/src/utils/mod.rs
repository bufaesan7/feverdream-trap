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

pub fn despawn_after(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnAfter)>,
) {
    for (entity, mut despawn_after) in &mut query {
        if despawn_after.0.tick(time.delta()).just_finished() {
            commands.entity(entity).try_despawn();
        }
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
pub struct Fade {
    mode: FadeMode,
    timer: Timer,
}

impl Fade {
    pub fn new(mode: FadeMode, duration: Duration) -> Self {
        Self {
            mode,
            timer: Timer::new(duration, TimerMode::Once),
        }
    }

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let Some(fade) = world.entity(ctx.entity).get::<Self>() else {
            return;
        };
        let mode = fade.mode;
        let duration = fade.timer.duration();

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

pub fn fade_text(time: Res<Time>, mut query: Query<(&mut TextColor, &mut Fade)>) {
    for (mut color, mut fade) in &mut query {
        fade.timer.tick(time.delta());

        let fraction = fade.timer.fraction();
        let fraction = match fade.mode {
            FadeMode::In => fraction,
            FadeMode::Out => 1.0 - fraction,
        };

        color.0.set_alpha(fraction);
    }
}

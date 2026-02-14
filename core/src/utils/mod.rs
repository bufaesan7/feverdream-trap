pub mod audio;
pub mod cursor;

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

#[derive(Debug, Reflect)]
pub enum FadeMode {
    In,
    Out,
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
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

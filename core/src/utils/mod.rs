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

    pub fn from_secs_f32(secs: f32) -> Self {
        Self::new(Duration::from_secs_f32(secs))
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

pub mod audio;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_after.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
}

// Schedule this so it only runs when paused
pub fn despawn_after(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnAfter)>,
) {
    for (entity, mut despawn_after) in &mut query {
        despawn_after_internal(&mut commands, entity, &mut despawn_after, time.delta());
    }
}

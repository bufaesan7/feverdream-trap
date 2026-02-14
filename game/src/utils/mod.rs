pub mod audio;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (despawn_after, fade_text).run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
}

// Schedule this so it only runs when not paused
pub fn despawn_after(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnAfter)>,
) {
    for (entity, mut despawn_after) in &mut query {
        despawn_after_internal(&mut commands, entity, &mut despawn_after, time.delta());
    }
}

// Schedule this so it only runs when not paused
pub fn fade_text(time: Res<Time>, mut query: Query<(&mut TextColor, &mut FadeText)>) {
    fade_text_internal(&mut query, time.delta());
}

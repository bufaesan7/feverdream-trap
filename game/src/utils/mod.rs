pub mod audio;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (despawn_after, fade_text, fade_sound)
            .run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
}

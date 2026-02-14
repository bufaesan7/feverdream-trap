pub mod audio;

use feverdream_trap_core::prelude::audio::fade_sound;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(audio::plugin).add_systems(
        Update,
        (despawn_after, fade_text, fade_sound).run_if(
            in_state(Screen::Gameplay)
                .and(in_state(Menu::None))
                .or(not(in_state(Screen::Gameplay))),
        ),
    );
}

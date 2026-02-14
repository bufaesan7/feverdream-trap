pub mod audio;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(audio::plugin)
        .add_systems(
            Update,
            (despawn_after, fade_text).run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        )
        .add_systems(Update, fade_sound); // A bit weird, but we want to fade out the music from the gameplay on the title screen, but also means it will not be despawned until later since that only runs in Gameplay
}

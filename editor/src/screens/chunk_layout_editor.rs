use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::ChunkLayoutEditor), spawn_editor);
}

#[cfg_attr(bevy_lint, allow(bevy::unit_in_bundle))]
fn spawn_editor(mut commands: Commands) {
    commands.spawn((
        widget::sidebar(
            (),
            children![
                widget::label_sized("Chunk layout editor", 12.),
                widget::button_small(
                    "Return",
                    super::change_editor_state::<{ Screen::EditorMenu as u32 }>
                ),
            ],
            true,
        ),
        DespawnOnExit(Screen::ChunkLayoutEditor),
    ));
}

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(EditorState::ChunkLayoutEditor), spawn_editor);
}

fn spawn_editor(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Chunk layout editor root"),
        DespawnOnExit(EditorState::ChunkLayoutEditor),
        children![
            widget::label("Chunk layout editor"),
            widget::button(
                "Return",
                super::change_editor_state::<{ EditorState::EditorMenu as u32 }>
            ),
        ],
    ));
}

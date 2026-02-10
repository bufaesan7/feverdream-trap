use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(EditorState::ChunkEditor), spawn_editor);
}

fn spawn_editor(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Chunk editor root"),
        DespawnOnExit(EditorState::ChunkEditor),
        children![
            widget::label("Chunk editor"),
            widget::button(
                "Return",
                super::change_editor_state::<{ EditorState::EditorMenu as u32 }>
            ),
        ],
    ));
}

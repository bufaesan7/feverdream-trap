use crate::prelude::*;

mod chunk_editor;
mod chunk_layout_editor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((chunk_editor::plugin, chunk_layout_editor::plugin));

    app.add_systems(OnEnter(EditorState::EditorMenu), spawn_menu);
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Editor menu"),
        DespawnOnExit(EditorState::EditorMenu),
        children![
            widget::label("Editor"),
            widget::button(
                "Chunk editor",
                change_editor_state::<{ EditorState::ChunkEditor as u32 }>
            ),
            widget::button(
                "Chunk layout editor",
                change_editor_state::<{ EditorState::ChunkLayoutEditor as u32 }>
            ),
            widget::button("Quit", exit)
        ],
    ));
}

fn change_editor_state<const STATE: u32>(
    _: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<EditorState>>,
) {
    next_state.set(unsafe { std::mem::transmute::<u32, EditorState>(STATE) });
}

fn exit(_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

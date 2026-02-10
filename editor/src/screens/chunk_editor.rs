use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::ChunkEditor), spawn_editor);
}

fn spawn_editor(mut commands: Commands, chunks: Res<Assets<ChunkDescriptor>>) {
    commands.spawn((
        widget::sidebar(
            children![widget::button_small("New chunk", new_chunk)],
            children![
                widget::label_sized("Chunk editor", 12.),
                widget::label_sized(format!("{} ChunkDescriptors loaded", chunks.len()), 12.),
                widget::button_small(
                    "Return",
                    super::change_editor_state::<{ Screen::EditorMenu as u32 }>
                ),
            ],
            true,
        ),
        DespawnOnExit(Screen::ChunkEditor),
    ));
}

fn new_chunk(_: On<Pointer<Click>>) {
    debug!("New chunk!");
}

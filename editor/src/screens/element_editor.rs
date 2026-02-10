use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::ElementEditor), spawn_editor);
}

fn spawn_editor(mut commands: Commands, chunks: Res<Assets<ChunkElement>>) {
    commands.spawn((
        widget::sidebar(
            children![widget::button_small("New chunk element", new_element)],
            children![
                widget::label_sized("Element editor", 12.),
                widget::label_sized(format!("{} ChunkElements loaded", chunks.len()), 12.),
                widget::button_small(
                    "Return",
                    super::change_editor_state::<{ Screen::EditorMenu as u32 }>
                ),
            ],
            true,
        ),
        DespawnOnExit(Screen::ElementEditor),
    ));
}

fn new_element(_: On<Pointer<Click>>) {
    debug!("New chunk!");
}

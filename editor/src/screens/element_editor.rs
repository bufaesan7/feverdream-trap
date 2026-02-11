use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::ElementEditor), spawn_editor);
}

fn spawn_editor(mut commands: Commands, chunks: Res<Assets<ChunkElement>>) {
    commands.spawn((
        editor_widget::sidebar(
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

fn new_element(_: On<Pointer<Click>>, mut commands: Commands) {
    editor_widget::item_creation(
        &mut commands,
        "ChunkElement",
        |p| {
            p.spawn((
                editor_widget::ui_row(),
                children![
                    widget::label_sized("Name:", 12.),
                    (
                        TextEditable::default(),
                        Text::new("Name"),
                        TextFont::from_font_size(12.)
                    )
                ],
            ));
            p.spawn(editor_widget::transform_editor());
        },
        element_created,
    );
}

fn element_created() {
    debug!("Element created!");
}

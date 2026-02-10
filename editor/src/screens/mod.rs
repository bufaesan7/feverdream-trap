use crate::prelude::*;

mod chunk_editor;
mod chunk_layout_editor;
mod element_editor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        element_editor::plugin,
        chunk_editor::plugin,
        chunk_layout_editor::plugin,
    ));

    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
    app.add_systems(Update, enter_menu_state.run_if(in_state(Screen::Loading)));

    app.add_systems(OnEnter(Screen::EditorMenu), spawn_menu);
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Editor loading screen"),
        DespawnOnExit(Screen::Loading),
        children![widget::label("Loading..."),],
    ));
}

fn enter_menu_state(
    resource_handles: Res<ResourceHandles>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_state.set(Screen::EditorMenu);
    }
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Editor menu"),
        DespawnOnExit(Screen::EditorMenu),
        children![
            widget::label("Editor"),
            widget::button(
                "Element editor",
                change_editor_state::<{ Screen::ElementEditor as u32 }>
            ),
            widget::button(
                "Chunk editor",
                change_editor_state::<{ Screen::ChunkEditor as u32 }>
            ),
            widget::button(
                "Chunk layout editor",
                change_editor_state::<{ Screen::ChunkLayoutEditor as u32 }>
            ),
            widget::button("Quit", exit)
        ],
    ));
}

fn change_editor_state<const STATE: u32>(
    _: On<Pointer<Click>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    next_state.set(unsafe { std::mem::transmute::<u32, Screen>(STATE) });
}

fn exit(_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy_text_edit::TextEditPluginAnyState;

use crate::prelude::*;

mod egui_layout;
mod elements;
mod prelude;

fn main() -> AppExit {
    let mut app = App::new();

    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins.set(asset_plugin()).set(WindowPlugin {
            primary_window: Window {
                title: "Feverdream Trap Editor".to_string(),
                name: Some("feverdream_trap_editor".to_string()),
                fit_canvas_to_parent: true,
                ..default()
            }
            .into(),
            ..default()
        }),
    );

    app.add_plugins(TextEditPluginAnyState::any());

    app.init_state::<Screen>();

    app.add_plugins((
        feverdream_trap_core::utility_plugin,
        egui_layout::plugin,
        elements::plugin,
    ));

    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
    app.add_systems(Update, enter_menu_state.run_if(in_state(Screen::Loading)));

    app.run()
}

#[derive(States, Hash, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(u32)]
enum Screen {
    #[default]
    Loading,
    Editor,
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
        next_state.set(Screen::Editor);
    }
}

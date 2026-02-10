use crate::prelude::*;

mod prelude;
mod screens;

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

    app.init_state::<EditorState>();

    app.add_plugins((feverdream_trap_core::utility_plugin, screens::plugin));

    app.add_systems(Startup, spawn_2d_camera);

    app.run()
}

#[derive(States, Hash, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(u32)]
enum EditorState {
    #[default]
    EditorMenu,
    ChunkEditor,
    ChunkLayoutEditor,
}

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera 2D"), Transform::default(), Camera2d));
}

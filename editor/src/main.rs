use feverdream_trap_core::prelude::*;

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

    app.add_systems(Startup, (spawn_2d_camera, spawn_menu));

    app.run()
}

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera 2D"), Transform::default(), Camera2d));
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Editor menu"),
        children![widget::label("Editor")],
    ));
}

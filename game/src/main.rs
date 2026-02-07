use crate::{
    camera_controller::CameraControllerPlugin, character_controller::CharacterControllerPlugin,
    prelude::*,
};

mod camera_controller;
mod character_controller;
mod prelude;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Default, Hash)]
enum AppState {
    #[cfg_attr(not(feature = "dev"), default)]
    MainMenu,
    Loading,
    #[cfg_attr(feature = "dev", default)]
    InGame,
}

fn main() {
    let mut app = App::new();

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Feverdream Trap".to_string(),
            name: Some("feverdream_trap".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    // Default plugins
    app.add_plugins(DefaultPlugins.set(window_plugin));

    // Ecosystem plugins
    app.add_plugins(PhysicsPlugins::default());

    // Custom game plugins
    app.add_plugins((CameraControllerPlugin, CharacterControllerPlugin));

    // App states
    app.init_state::<AppState>();

    app.run();
}

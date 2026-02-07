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

    app.add_systems(OnEnter(AppState::InGame), demo_scene);

    app.run();
}

fn demo_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Plane"),
        Transform::default(),
        Visibility::Visible,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.)))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
    ));

    commands.spawn((
        Name::new("Cube"),
        Transform::from_xyz(0., 0., -20.),
        Visibility::Visible,
        Mesh3d(meshes.add(Cuboid::new(3., 3., 3.))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(
            bevy::color::palettes::css::BLUE,
        ))),
    ));
}

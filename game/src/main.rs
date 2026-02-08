// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod camera_controller;
mod character_controller;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod prelude;
mod screens;
mod theme;

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use feverdream_trap_core::prelude::*;

use crate::{
    camera_controller::CameraControllerPlugin, character_controller::CharacterControllerPlugin,
};

#[allow(dead_code)]
#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Default, Hash)]
enum AppState {
    #[cfg_attr(not(feature = "dev"), default)]
    MainMenu,
    Loading,
    #[cfg_attr(feature = "dev", default)]
    InGame,
}

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins.set(asset_plugin()).set(WindowPlugin {
                primary_window: Window {
                    title: "Feverdream Trap".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            demo::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Ecosystem plugins
        app.add_plugins(PhysicsPlugins::default());

        // Custom game plugins
        app.add_plugins((CameraControllerPlugin, CharacterControllerPlugin));

        // App states
        app.init_state::<AppState>();

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        app.add_systems(OnEnter(AppState::InGame), demo_scene);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
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

// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod camera_controller;
mod character_controller;
#[cfg(feature = "dev")]
mod dev_tools;
mod interaction;
mod level;
mod menus;
mod prelude;
mod scene;
mod screens;
mod theme;

use avian3d::PhysicsPlugins;

use crate::{
    camera_controller::CameraControllerPlugin, character_controller::CharacterControllerPlugin,
    prelude::*,
};

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
                    name: Some("feverdream_trap".to_string()),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        );

        // Ecosystem plugins
        app.add_plugins(PhysicsPlugins::default());

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            scene::plugin,
            theme::plugin,
            interaction::plugin,
        ));

        // Custom game plugins
        app.add_plugins((CameraControllerPlugin, CharacterControllerPlugin));

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
        app.configure_sets(
            Update,
            PausableSystems.run_if(in_state(Pause(false)).and(in_state(Screen::Gameplay))),
        );
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

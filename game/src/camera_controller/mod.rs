use bevy::window::{CursorGrabMode, CursorOptions};

use crate::prelude::*;

mod setup;

pub use setup::spawn_camera;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            sensivity: Vec2::splat(0.001),
        });
        app.add_systems(OnEnter(Screen::Gameplay), cursor_grab)
            .add_systems(OnExit(Screen::Gameplay), cursor_ungrab);
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct CameraSettings {
    /// Motion sensitivity, determines the rotation speed.
    sensivity: Vec2,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Camera3d::default(),
    PrimaryEguiContext = PrimaryEguiContext,
    DistanceFog {
        color: Color::srgb(0.25, 0.25, 0.25),
        falloff: FogFalloff::Linear {
            start: 5.0,
            end: 250.0,
        },
        ..Default::default()
    },
)]
pub struct CameraMarker;

pub fn cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn cursor_ungrab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}

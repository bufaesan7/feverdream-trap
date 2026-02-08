use bevy::window::{CursorGrabMode, CursorOptions};

use crate::{camera_controller::post_process::PostProcessPlugin, prelude::*};

mod post_process;
mod rotate;
mod setup;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PostProcessPlugin);

        app.add_systems(Startup, setup::spawn_camera)
            .add_systems(OnEnter(Screen::Gameplay), setup::activate_camera)
            .add_systems(OnExit(Screen::Gameplay), setup::deactivate_camera)
            .add_systems(Update, rotate::rotate_camera.in_set(PausableSystems));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraMarker {
    /// Motion sensitivity, determines the rotation speed.
    pub sensivity: Vec2,
}

pub fn cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn cursor_ungrab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}

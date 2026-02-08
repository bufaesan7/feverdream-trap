use crate::camera_controller::post_process::PostProcessSettings;
use crate::prelude::*;

use crate::{camera_controller::CameraMarker, character_controller::CharacterController};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Transform::default(),
        Visibility::Visible,
        CameraMarker {
            // Might need tweaking, I only tested on a touchpad
            sensivity: Vec2::splat(0.001),
        },
        CharacterController,
        Camera {
            is_active: false,
            order: -1,
            ..Default::default()
        },
        Camera3d::default(),
        PostProcessSettings {
            intensity: 0.02,
            ..default()
        },
    ));
}

pub fn activate_camera(mut camera: Single<(&mut Transform, &mut Camera), With<CameraMarker>>) {
    *camera.0 = Transform::default();
    camera.1.is_active = true;
}

pub fn deactivate_camera(mut camera: Single<&mut Camera, With<CameraMarker>>) {
    camera.is_active = false;
}

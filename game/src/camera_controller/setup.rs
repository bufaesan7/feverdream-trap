use bevy::prelude::*;

use crate::{camera_controller::CameraMarker, character_controller::CharacterController};

pub fn spawn_camera(mut commands: Commands) {
    debug!("spawn camera");
    commands.spawn((
        Name::new("Camera"),
        Transform::default(),
        Visibility::Visible,
        CameraMarker {
            // Might need tweaking, I only tested on a touchpad
            sensivity: -Vec2::splat(0.001),
        },
        CharacterController,
        Camera3d::default(),
    ));
}

pub fn reset_camera(mut transform: Single<&mut Transform, With<CameraMarker>>) {
    **transform = Transform::default();
}

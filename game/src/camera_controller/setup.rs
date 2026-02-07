use bevy::prelude::*;

use crate::camera_controller::CameraMarker;

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
        Camera3d::default(),
    ));
}

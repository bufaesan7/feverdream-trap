use bevy_ahoy::camera::CharacterControllerCameraOf;

use crate::prelude::*;

use crate::camera_controller::CameraMarker;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Transform::default(),
        Visibility::Visible,
        CameraMarker,
        Camera3d::default(),
        Camera {
            order: -1,
            ..Default::default()
        },
    ));
}

pub fn remove_camera_target_of(mut commands: Commands, camera: Single<Entity, With<CameraMarker>>) {
    commands
        .entity(*camera)
        .remove::<CharacterControllerCameraOf>();
}

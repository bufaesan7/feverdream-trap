use crate::prelude::*;

use crate::camera_controller::CameraMarker;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Transform::default(),
        Visibility::Visible,
        CameraMarker,
        LevelComponent,
    ));
}

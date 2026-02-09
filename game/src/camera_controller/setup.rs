use crate::level::LevelComponent;
use crate::prelude::*;

use crate::{camera_controller::CameraMarker, character_controller::CharacterController};

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Character"),
            Transform::default(),
            Visibility::Visible,
            CharacterController,
            LevelComponent,
        ))
        .with_child((
            Name::new("Camera"),
            Transform::default(),
            Visibility::Visible,
            CameraMarker,
            LevelComponent,
        ));
}

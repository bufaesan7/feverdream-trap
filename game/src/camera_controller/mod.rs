use crate::prelude::*;

mod rotate;
mod setup;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        // TODO! run_if InGame or in a game set
        app.add_systems(OnEnter(AppState::InGame), setup::spawn_camera)
            .add_systems(Update, rotate::rotate_camera);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraMarker {
    /// Motion sensivity, determines the rotation speed.
    pub sensivity: Vec2,
}

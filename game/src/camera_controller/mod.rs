use crate::prelude::*;

mod rotate;
mod setup;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::spawn_camera).add_systems(
            Update,
            rotate::rotate_camera.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        );
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraMarker {
    /// Motion sensivity, determines the rotation speed.
    pub sensivity: Vec2,
}

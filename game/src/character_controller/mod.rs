use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{camera_controller::CameraMarker, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnhancedInputPlugin, AhoyPlugins::default()))
            .add_input_context::<PlayerInput>();
    }
}

#[derive(Component)]
struct PlayerInput;

pub fn spawn_player(mut commands: Commands, camera: Single<Entity, With<CameraMarker>>) {
    // Spawn the player entity
    let player = commands
        .spawn((
            // The character controller configuration
            CharacterController::default(),
            // The KCC currently behaves best when using a cylinder
            Collider::cylinder(0.7, 1.8),
            Transform::from_xyz(0.0, 20.0, 0.0),
            // Configure inputs
            PlayerInput,
            actions!(PlayerInput[
                (
                    Action::<Movement>::new(),
                    DeadZone::default(),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                    ))
                ),
                (
                    Action::<RotateCamera>::new(),
                    Scale::splat(0.04),
                    Bindings::spawn((
                        Spawn(Binding::mouse_motion()),
                        Axial::right_stick()
                    ))
                ),
            ]),
        ))
        .id();

    // Spawn the camera
    commands
        .entity(*camera)
        .insert(CharacterControllerCameraOf::new(player));
}

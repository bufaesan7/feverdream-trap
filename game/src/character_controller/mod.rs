use crate::{camera_controller::CameraMarker, level::LevelComponent, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            character_control.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        );
    }
}

/// This is a marker component for the Player
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(LevelComponent, CharacterController)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, camera: Single<Entity, With<CameraMarker>>) {
    // Spawn the player entity
    commands
        .spawn((
            Collider::cylinder(0.7, 1.8),
            RigidBody::Kinematic,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player,
        ))
        .add_child(*camera);
}

const CHARACTER_VELOCITY: f32 = 50.0;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct CharacterController;

fn character_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut velocities: Query<&mut LinearVelocity, With<CharacterController>>,
) {
    for mut velocity in &mut velocities {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        velocity.0 = direction.normalize_or_zero() * CHARACTER_VELOCITY;
    }
}

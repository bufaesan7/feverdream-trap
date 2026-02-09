use crate::prelude::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            character_controller.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        );
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct CharacterController;

// TODO: Replace with avian velocity
const VELOCITY: f32 = 100.0;

fn character_controller(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut controller: Query<&mut Transform, With<CharacterController>>,
) {
    let mut direction = Vec3::ZERO;

    for mut transform in &mut controller {
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);

        if key_input.pressed(KeyCode::KeyW) {
            direction += forward;
        }
        if key_input.pressed(KeyCode::KeyS) {
            direction -= forward;
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * VELOCITY * time.delta_secs();
    }
}

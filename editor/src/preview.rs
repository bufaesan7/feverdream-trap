use bevy::input::{
    common_conditions::{input_just_released, input_pressed},
    mouse::AccumulatedMouseMotion,
};
use feverdream_trap_core::prelude::cursor::{cursor_grab, cursor_ungrab};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EditorPreview>();
    app.add_systems(
        Update,
        (
            update_descriptor_preview.chain(),
            rotate_camera
                .run_if(input_pressed(MouseButton::Middle).or(input_pressed(MouseButton::Right))),
            cursor_grab.run_if(
                input_just_pressed(MouseButton::Middle).or(input_just_pressed(MouseButton::Right)),
            ),
            cursor_ungrab.run_if(
                input_just_released(MouseButton::Middle)
                    .or(input_just_released(MouseButton::Right)),
            ),
            move_camera,
        )
            .run_if(in_state(Screen::Editor)),
    );
}

#[derive(Resource, Default)]
pub enum EditorPreview {
    #[default]
    Layout,
    Descriptor(Handle<ChunkDescriptor>),
}

fn update_descriptor_preview(
    mut commands: Commands,
    level: Query<Entity, With<Level>>,
    preview: Res<EditorPreview>,
    descriptor_assets: Res<Assets<ChunkDescriptor>>,
    element_assets: Res<Assets<ChunkElement>>,
) {
    // Despawn previous preview
    if let Ok(level_entity) = level.single() {
        // Despawn the previous level entity
        commands.entity(level_entity).despawn();
    }

    match &*preview {
        EditorPreview::Descriptor(descriptor_handle) => {
            let level_entity = commands
                .spawn((
                    Name::new("Preview Level"),
                    Level,
                    Transform::default(),
                    Visibility::default(),
                ))
                .id();

            let Some(descriptor) = descriptor_assets.get(descriptor_handle) else {
                return;
            };

            let elements: Vec<ChunkElement> = descriptor
                .elements
                .iter()
                .filter_map(|wrapper| element_assets.get(&wrapper.0).cloned())
                .collect();

            if elements.is_empty() {
                return;
            }

            commands.trigger(SpawnChunk {
                level: level_entity,
                id: ChunkId(0),
                grid_position: Vec2::ZERO,
                descriptor: descriptor_handle.clone(),
                components: vec![],
            });
        }
        EditorPreview::Layout => commands.run_system_cached(spawn_level_from_layout),
    }
}

/// <https://bevy.org/examples/camera/camera-orbit/>
fn rotate_camera(
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok(mut camera) = camera.single_mut() else {
        return;
    };

    let delta = -mouse_motion.delta;
    let delta_pitch = delta.y * 0.003;
    let delta_yaw = delta.x * 0.004;

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch = pitch + delta_pitch;
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    // In our example it's a static target, but this could easily be customized.
    let target = Vec3::ZERO;
    camera.translation = target - camera.forward() * CHUNK_SIZE * 4.;
}

#[derive(Component)]
/// The ankor point of the camera (the camera will rotate around this center)
pub(super) struct CameraAnkor;

fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut ankor: Single<&mut Transform, With<CameraAnkor>>,
    camera: Single<&Transform, (With<Camera3d>, Without<CameraAnkor>)>,
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction += Vec3::NEG_Z;
    }
    if input.pressed(KeyCode::KeyA) {
        direction += Vec3::NEG_X;
    }
    if input.pressed(KeyCode::KeyS) {
        direction += Vec3::Z;
    }
    if input.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }
    if input.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        direction += Vec3::NEG_Y;
    }
    let speed = 10.;
    ankor.translation += Quat::from_rotation_arc(
        Vec3::NEG_Z,
        camera.forward().as_vec3().with_y(0.).normalize_or_zero(),
    ) * direction
        * time.delta_secs()
        * speed;
}

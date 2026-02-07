use bevy::input::mouse::AccumulatedMouseMotion;

use crate::{camera_controller::CameraMarker, prelude::*};

pub fn rotate_camera(
    motion: Res<AccumulatedMouseMotion>,
    mut camera: Query<(&mut Transform, &CameraMarker)>,
) {
    let delta = motion.delta;

    // https://github.com/bevyengine/bevy/blob/133c8abe19443be1f2d1d35cbc28f8dd4bacdfe2/examples/camera/first_person_view_model.rs#L213
    if delta != Vec2::ZERO
        && let Ok((mut transform, settings)) = camera.single_mut()
    {
        let delta_yaw = -delta.x * settings.sensivity.x;
        let delta_pitch = -delta.y * settings.sensivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

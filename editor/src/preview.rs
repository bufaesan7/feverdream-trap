use bevy::input::{
    common_conditions::{input_just_released, input_pressed},
    mouse::AccumulatedMouseMotion,
};
use feverdream_trap_core::prelude::cursor::{cursor_grab, cursor_ungrab};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DescriptorPreview>();
    app.add_systems(
        Update,
        (
            (refresh_preview_on_asset_change, update_descriptor_preview).chain(),
            rotate_camera.run_if(input_pressed(MouseButton::Middle)),
            cursor_grab.run_if(input_just_pressed(MouseButton::Middle)),
            cursor_ungrab.run_if(input_just_released(MouseButton::Middle)),
        )
            .run_if(in_state(Screen::Editor)),
    );
}

#[derive(Resource, Default)]
pub struct DescriptorPreview {
    pub descriptor: Option<Handle<ChunkDescriptor>>,
    level_entity: Option<Entity>,
}

fn refresh_preview_on_asset_change(
    mut element_events: MessageReader<AssetEvent<ChunkElement>>,
    mut descriptor_events: MessageReader<AssetEvent<ChunkDescriptor>>,
    mut preview: ResMut<DescriptorPreview>,
) {
    let Some(preview_handle) = preview.descriptor.as_ref() else {
        element_events.clear();
        descriptor_events.clear();
        return;
    };

    let mut needs_refresh = false;

    for event in element_events.read() {
        if matches!(event, AssetEvent::Modified { .. }) {
            needs_refresh = true;
        }
    }

    let preview_id = preview_handle.id();
    for event in descriptor_events.read() {
        if let AssetEvent::Modified { id } = event
            && *id == preview_id
        {
            needs_refresh = true;
        }
    }

    if needs_refresh {
        preview.set_changed();
    }
}

fn update_descriptor_preview(
    mut commands: Commands,
    mut preview: ResMut<DescriptorPreview>,
    descriptor_assets: Res<Assets<ChunkDescriptor>>,
    element_assets: Res<Assets<ChunkElement>>,
) {
    if !preview.is_changed() {
        return;
    }

    // Despawn previous preview
    if let Some(level_entity) = preview.level_entity.take() {
        // Despawn the chunk via the DespawnChunk observer
        commands.trigger(DespawnChunk(ChunkId(0)));
        // Despawn the level entity itself
        commands.entity(level_entity).despawn();
    }

    // Spawn new preview if a descriptor is selected
    let Some(descriptor_handle) = preview.descriptor.as_ref() else {
        return;
    };

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

    let level_entity = commands
        .spawn((
            Name::new("Preview Level"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    commands.trigger(SpawnChunk {
        level: level_entity,
        id: ChunkId(0),
        grid_position: Vec2::ZERO,
        descriptor: descriptor_handle.clone(),
        #[cfg(feature = "dev")]
        show_wireframe: true,
    });

    preview.level_entity = Some(level_entity);
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

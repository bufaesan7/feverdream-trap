//! Development tools for the game. This plugin is only enabled in dev builds.

use std::any::TypeId;

use crate::{camera_controller::CameraMarker, prelude::*};
use bevy::{
    dev_tools::{
        fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
        states::log_transitions,
    },
    input::common_conditions::input_toggle_active,
    window::{CursorGrabMode, CursorOptions},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: false,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: false,
                ..Default::default()
            },
            ..Default::default()
        },
    })
    .add_plugins(EguiPlugin::default())
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, TOGGLE_INSPECTOR_KEY)),
    )
    .add_plugins(PhysicsDebugPlugin)
    .insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::WHITE),
            ..default()
        },
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle cursor grab for inspector
    app.add_systems(
        Update,
        toggle_cursor_grab.run_if(input_just_pressed(TOGGLE_INSPECTOR_KEY)),
    );

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_DEBUG_UI_KEY)),
    );

    app.add_systems(
        Update,
        toggle_fps_overlay.run_if(input_just_pressed(TOGGLE_FPS_OVERLAY_KEY)),
    );

    app.add_systems(
        Update,
        toggle_fog.run_if(input_just_pressed(TOGGLE_FOG_KEY)),
    );

    app.add_systems(
        Update,
        toggle_physics_gizmos.run_if(input_just_pressed(TOGGLE_PHYSICS_GIZMOS_KEY)),
    );
}

const TOGGLE_INSPECTOR_KEY: KeyCode = KeyCode::F1;

const TOGGLE_DEBUG_UI_KEY: KeyCode = KeyCode::F2;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

const TOGGLE_FPS_OVERLAY_KEY: KeyCode = KeyCode::F3;

fn toggle_fps_overlay(mut config: ResMut<FpsOverlayConfig>) {
    config.enabled = !config.enabled;
    config.frame_time_graph_config.enabled = !config.frame_time_graph_config.enabled;
}

const TOGGLE_FOG_KEY: KeyCode = KeyCode::F4;

fn toggle_fog(
    mut commands: Commands,
    fog: Single<(Entity, Option<&DistanceFog>), With<CameraMarker>>,
    mut previous: Local<DistanceFog>,
) {
    let (entity, fog) = *fog;

    if let Some(fog) = fog {
        *previous = fog.clone();
        commands.entity(entity).remove::<DistanceFog>();
    } else {
        commands.entity(entity).insert(previous.clone());
    }
}

const TOGGLE_PHYSICS_GIZMOS_KEY: KeyCode = KeyCode::F5;

fn toggle_physics_gizmos(mut gizmo: ResMut<GizmoConfigStore>) {
    if let Some((config, _)) = gizmo.get_config_mut_dyn(&TypeId::of::<PhysicsGizmos>()) {
        config.enabled = !config.enabled;
    }
}

fn toggle_cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = !cursor_options.visible;

    cursor_options.grab_mode = match cursor_options.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        _ => CursorGrabMode::None,
    }
}

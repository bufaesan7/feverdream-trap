use crate::{character_controller::CharacterController, prelude::*};
use bevy::{
    input::mouse::MouseMotion,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

mod setup;

pub use setup::spawn_camera;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            sensivity: Vec2::splat(0.001),
        });
        app.add_systems(OnEnter(Screen::Gameplay), cursor_grab)
            .add_systems(OnExit(Screen::Gameplay), cursor_ungrab)
            .add_systems(
                Update,
                camera_control.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
            );
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct CameraSettings {
    /// Motion sensitivity, determines the rotation speed.
    sensivity: Vec2,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    Camera3d::default(),
    LevelComponent,
    PrimaryEguiContext = PrimaryEguiContext,
    DistanceFog {
        color: Color::srgb(0.25, 0.25, 0.25),
        falloff: FogFalloff::Linear {
            start: 5.0,
            end: 250.0,
        },
        ..Default::default()
    },
    CameraController,
)]
pub struct CameraMarker;

pub fn cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn cursor_ungrab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct CameraController;

//https://github.com/sburris0/bevy_flycam/blob/master/src/lib.rs
fn camera_control(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut state: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &ChildOf), With<CameraController>>,
    mut character_rotation: Query<&mut Rotation, With<CharacterController>>,
) {
    let sensitivity = 0.00012;
    if let Ok(window) = primary_window.single() {
        for (mut transform, child_of) in query.iter_mut() {
            if let Ok(mut character_rotation) = character_rotation.get_mut(child_of.parent()) {
                let (mut yaw, _, _) = character_rotation.0.to_euler(EulerRot::YXZ);
                for ev in state.read() {
                    let (_, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                    match primary_cursor_options.grab_mode {
                        CursorGrabMode::None => (),
                        _ => {
                            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                            let window_scale = window.height().min(window.width());
                            pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
                            yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();
                        }
                    }

                    pitch = pitch.clamp(-1.54, 1.54);

                    transform.rotation = Quat::from_axis_angle(Vec3::X, pitch);
                    character_rotation.0 = Quat::from_axis_angle(Vec3::Y, yaw);
                }
            }
        }
    } else {
        warn!("Primary window not found for `camera_control`!");
    }
}

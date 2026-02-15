use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
use bevy_ahoy::camera::CharacterControllerCameraOf;
use feverdream_trap_core::prelude::cursor::{cursor_grab, cursor_ungrab};

use crate::prelude::*;

mod screen_darken;
mod setup;
mod status_effects;

pub use setup::spawn_camera;
pub use status_effects::CameraStatusEffects;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((status_effects::plugin, screen_darken::plugin));

        app.insert_resource(CameraSettings {
            sensivity: Vec2::splat(0.001),
        });
        app.add_systems(OnEnter(Screen::Gameplay), cursor_grab)
            .add_systems(OnExit(Screen::Gameplay), cursor_ungrab);
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
    screen_darken::ScreenDarkenEffect,
)]
#[component(on_add = status_effects::add_camera_effects)]
pub struct CameraMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[relationship(relationship_target = CharacterControllerCameraTarget)]
/// Needed because [`CharacterControllerCameraOf`] does not implement [`Reflect`]
/// This component on the camera points to the player
pub struct CameraTargetCharacterController(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = CameraTargetCharacterController)]
/// Component on the player marking it as the target of the camera
pub struct CharacterControllerCameraTarget(Vec<Entity>);

impl CameraTargetCharacterController {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        let target = world
            .get::<CameraTargetCharacterController>(hook.entity)
            .unwrap()
            .0;

        world
            .commands()
            .entity(hook.entity)
            .insert(CharacterControllerCameraOf::new(target));
    }
}

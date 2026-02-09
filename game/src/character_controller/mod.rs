use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{camera_controller::CameraMarker, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnhancedInputPlugin, AhoyPlugins::default()))
            .add_input_context::<PlayerInput>()
            .add_systems(
                OnEnter(Menu::None),
                add_player_input.run_if(in_state(Screen::Gameplay)),
            )
            .add_systems(
                OnExit(Menu::None),
                remove_player_input.run_if(in_state(Screen::Gameplay)),
            );
    }
}

/// This is a marker component for the Player
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct Player;

/// This marker component is registered with bevy_ahoy/bevy_enhanced_input
/// to drive the input->movement.
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = PlayerInput::on_add)]
struct PlayerInput;

impl PlayerInput {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world
            .commands()
            .entity(ctx.entity)
            .insert(actions!(PlayerInput[
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
            ]));
    }
}

pub fn spawn_player(mut commands: Commands, camera: Single<Entity, With<CameraMarker>>) {
    // Spawn the player entity
    let player = commands
        .spawn((
            // The character controller configuration
            CharacterController::default(),
            // The KCC currently behaves best when using a cylinder
            Collider::cylinder(0.7, 1.8),
            Transform::from_xyz(50., 1., 70.),
            Player,
            PlayerInput,
        ))
        .id();

    // Spawn the camera
    commands
        .entity(*camera)
        .insert(CharacterControllerCameraOf::new(player));
}

// PlayerInput needs to be removed if Screen::Gameplay + (Event)Menu::Pause
// PlayerInput needs to be added if Screen::Gameplay + (Event)Menu::None
fn add_player_input(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands.entity(*player).insert(PlayerInput);
}

fn remove_player_input(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands
        .entity(*player)
        .remove_with_requires::<PlayerInput>()
        .despawn_related::<Actions<PlayerInput>>();
}

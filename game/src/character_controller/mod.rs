use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;
use feverdream_trap_core::physics::GameLayer;

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
#[require(
    LevelComponent,
    // The character controller configuration
    CharacterController::default(),
    // The KCC currently behaves best when using a cylinder
    Collider::cylinder(0.7, 1.8),
    CollisionLayers::new([GameLayer::Player], [GameLayer::Default, GameLayer::Sensor]),
    // Having this be a normal collider will conflict with ahoy and result in buggy movement
    Sensor
)]
#[component(on_add)]
pub struct Player;

impl Player {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let Some(mut query_state) = world.try_query_filtered() else {
            return;
        };
        let query = world.query::<Entity, With<CameraMarker>>(&mut query_state);
        let Ok(camera) = query.single() else {
            return;
        };
        world
            .commands()
            .entity(camera)
            .insert(CharacterControllerCameraOf::new(ctx.entity));
    }
}

/// This marker component is registered with bevy_ahoy/bevy_enhanced_input
/// to drive the input->movement.
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
pub struct PlayerInput;

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

pub fn spawn_player(
    mut commands: Commands,
    spawn_point: Query<(&Transform, &SpawnMarker), With<Chunk>>,
) {
    // Retrieve the spawn marker transform
    let spawn_transform = match spawn_point.single() {
        Ok((chunk_transform, SpawnMarker(point))) => {
            let chunk_position = chunk_transform.translation;

            info!("chunk pos: {}", chunk_position.xyz());
            info!("spawn pos: {}", point.translation.xyz());

            Transform::from_xyz(
                chunk_position.x + point.translation.x,
                1.0 + point.translation.y,
                chunk_position.z + point.translation.z,
            )
        }
        Err(_) => {
            warn!("spawn point not found");
            Transform::from_xyz(0.0, 1.0, 0.0)
        }
    };

    info!("Spawning player at {}", spawn_transform.translation.xyz());

    // Spawn the player entity
    commands.spawn((Name::new("Player"), spawn_transform, Player, PlayerInput));
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

use crate::{level::LevelComponent, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, _app: &mut App) {}
}

/// This is a marker component for the Player
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(LevelComponent)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    // Spawn the player entity
    commands.spawn((
        Collider::cylinder(0.7, 1.8),
        Transform::from_xyz(0.0, 20.0, 0.0),
        Player,
    ));
}

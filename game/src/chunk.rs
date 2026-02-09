use bevy::prelude::*;

#[derive(Default, Component)]
#[require(Transform, Visibility)]
pub struct Chunk;

use crate::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct DebugInteraction;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct DespawnInteraction;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct SwapChunksInteraction {
    pub chunk_a: u32,
    pub chunk_b: u32,
    pub persistent: bool,
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct PlaySoundEffectInteraction(pub String);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct PickupFuseInteraction;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct ElevatorInteraction;

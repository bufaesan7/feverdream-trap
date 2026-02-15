use crate::prelude::*;

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct DebugInteraction;

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct DespawnInteraction;

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct SwapChunksInteraction(pub ChunkId, pub ChunkId);

#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct PlaySoundEffectInteraction(pub String);

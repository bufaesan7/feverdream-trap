use crate::character_controller::Player;
use crate::prelude::*;
use std::mem::swap;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerChunk>()
            .add_observer(on_player_entered_chunk)
            .add_observer(on_swap_chunks)
            .add_systems(PreUpdate, (swap_chunks_on_contact_with_sensor,));
    }
}

#[derive(Default, Component, Reflect)]
#[require(Transform, Visibility, Sensor, LevelComponent)]
#[reflect(Component)]
pub struct Chunk;

#[derive(Default, Component, Reflect, Debug, Copy, Clone)]
#[require(Chunk)]
#[reflect(Component)]
pub struct ChunkId(pub u32);

/// swaps the two associated chunks on player entrance
#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct SwapSensorChunk(pub ChunkId, pub ChunkId);

#[derive(Debug, Event)]
pub struct SwapChunks(pub ChunkId, pub ChunkId);

/// the Chunk the player is currently in
#[derive(Resource, Default)]
pub struct PlayerChunk(pub Option<Entity>);

pub struct ChunkPlugin;

pub fn on_player_entered_chunk(
    event: On<CollisionStart>,
    player_query: Query<&Player>,
    chunk_query: Query<&ChunkId>,
    mut player_chunk: ResMut<PlayerChunk>,
) {
    let chunk = event.collider1;
    let player = event.collider2;

    if !player_query.contains(player) {
        return;
    }

    let Ok(ChunkId(chunk_id)) = chunk_query.get(chunk) else {
        return;
    };

    info!("Player {player} entered chunk {chunk_id}");

    player_chunk.0 = Some(chunk);
}

fn on_swap_chunks(
    event: On<SwapChunks>,
    mut chunk_transform_query: Query<(&ChunkId, &mut Transform)>,
) {
    let SwapChunks(ChunkId(chunk_a), ChunkId(chunk_b)) = *event;

    info!("Swapping chunk {:?} with {:?}", chunk_a, chunk_b);

    let mut chunk_transforms =
        chunk_transform_query
            .iter_mut()
            .filter_map(|(ChunkId(id), transform)| match id {
                chunk_id if [chunk_a, chunk_b].contains(chunk_id) => Some(transform),
                _ => None,
            });

    let (chunk_a_transform, chunk_b_transform) = (chunk_transforms.next(), chunk_transforms.next());

    if let (Some(mut transform_a), Some(mut transform_b)) = (chunk_a_transform, chunk_b_transform) {
        swap(&mut transform_a.translation, &mut transform_b.translation);
        info!("Swapped chunk {:?} with {:?}", chunk_a, chunk_b);
    }
}

fn swap_chunks_on_contact_with_sensor(
    mut commands: Commands,
    player_chunk: Res<PlayerChunk>,
    sensors_query: Query<(&SwapSensorChunk, &ChunkId)>,
) {
    let Some(chunk) = player_chunk.0 else { return };
    let Ok((SwapSensorChunk(chunk_a, chunk_b), ChunkId(chunk_id))) = sensors_query.get(chunk)
    else {
        return;
    };

    info!("Player triggered chunk swaps by entering sensor chunk {chunk_id}");

    commands.trigger(SwapChunks(*chunk_a, *chunk_b));
    commands.entity(chunk).remove::<SwapSensorChunk>();
}

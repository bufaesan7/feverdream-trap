use crate::character_controller::Player;
use feverdream_trap_core::prelude::*;
use std::mem::swap;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerChunk>()
            .add_observer(on_player_entered_chunk)
            .add_observer(on_replace_chunk_asset)
            .add_observer(on_swap_chunks)
            .add_systems(
                PreUpdate,
                (
                    swap_chunks_on_contact_with_sensor,
                    replace_chunk_asset_on_contact_with_sensor,
                ),
            );
    }
}

#[derive(Debug, Event)]
pub struct SwapChunks(pub ChunkId, pub ChunkId);
#[derive(Debug, Event)]
pub struct ReplaceChunkAsset(pub ChunkId, pub Handle<ChunkDescriptor>);

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

fn on_replace_chunk_asset(
    event: On<ReplaceChunkAsset>,
    mut commands: Commands,
    chunk_query: Query<(&ChunkId, &Transform, &ChildOf)>,
) {
    let ChunkId(chunk_id) = event.0;
    let chunk_asset = event.1.clone();

    let Some((chunk_transform, ChildOf(level))) =
        chunk_query
            .iter()
            .find_map(|(ChunkId(id), transform, child_of)| match *id {
                n if n == chunk_id => Some((transform, child_of)),
                _ => None,
            })
    else {
        return;
    };

    commands.trigger(DespawnChunk(ChunkId(chunk_id)));
    commands.trigger(SpawnChunk {
        level: *level,
        id: ChunkId(chunk_id),
        grid_position: chunk_transform.translation.xz() / CHUNK_SIZE,
        descriptor: chunk_asset.clone(),
        #[cfg(feature = "dev")]
        show_wireframe: false,
    });

    info!("Chunk {chunk_id} was replaced with {chunk_asset:?}");
}

fn replace_chunk_asset_on_contact_with_sensor(
    mut commands: Commands,
    player_chunk: Res<PlayerChunk>,
    sensors_query: Query<(&ReplaceAssetSensorChunk, &ChunkId)>,
) {
    let Some(player_chunk) = player_chunk.0 else {
        return;
    };

    let Ok((ReplaceAssetSensorChunk(chunk_to_replace, chunk_asset), ChunkId(chunk_id))) =
        sensors_query.get(player_chunk)
    else {
        return;
    };

    info!("Player triggered chunk asset replacement by entering sensor chunk {chunk_id}");

    commands.trigger(ReplaceChunkAsset(*chunk_to_replace, chunk_asset.clone()));
    commands
        .entity(player_chunk)
        .remove::<ReplaceAssetSensorChunk>();
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

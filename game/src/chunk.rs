use crate::character_controller::Player;
use feverdream_trap_core::prelude::*;
use std::mem::swap;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivePlayerChunks>()
            .add_observer(on_player_entered_chunk)
            .add_observer(on_player_exited_chunk)
            .add_observer(on_replace_chunk_asset)
            .add_observer(on_swap_chunks)
            .add_observer(on_move_chunk)
            .add_systems(
                PreUpdate,
                (
                    swap_chunks_on_contact_with_sensor,
                    replace_chunk_asset_on_contact_with_sensor,
                    move_chunk_on_contact_with_sensor,
                ),
            );
    }
}

#[derive(Debug, Event)]
pub struct SwapChunks(pub ChunkId, pub ChunkId);
#[derive(Debug, Event)]
pub struct ReplaceChunkAsset(pub ChunkId, pub Handle<ChunkDescriptor>);
#[derive(Debug, Event)]
pub struct MoveChunk(pub ChunkId, pub i32, pub i32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
/// Chunks the player is currently in
struct ActivePlayerChunks(Vec<ActivePlayerChunk>);
#[derive(Reflect)]
struct ActivePlayerChunk {
    chunk_entity: Entity,
    swap_sensor_activated: bool,
    asset_sensor_activated: bool,
    move_sensor_activated: bool,
}
impl ActivePlayerChunk {
    fn new(id: Entity) -> Self {
        Self {
            chunk_entity: id,
            swap_sensor_activated: false,
            asset_sensor_activated: false,
            move_sensor_activated: false,
        }
    }
}

pub struct ChunkPlugin;

fn on_player_entered_chunk(
    event: On<CollisionStart>,
    player_query: Query<&Player>,
    chunk_query: Query<&ChunkId>,
    mut player_chunk: ResMut<ActivePlayerChunks>,
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

    player_chunk.0.push(ActivePlayerChunk::new(chunk));
}

fn on_player_exited_chunk(
    event: On<CollisionEnd>,
    player_query: Query<&Player>,
    chunk_query: Query<&ChunkId>,
    mut player_chunk: ResMut<ActivePlayerChunks>,
) {
    let chunk = event.collider1;
    let player = event.collider2;

    if !player_query.contains(player) {
        return;
    }

    let Ok(ChunkId(chunk_id)) = chunk_query.get(chunk) else {
        return;
    };

    info!("Player {player} exited chunk {chunk_id}");

    player_chunk.0.retain(|active| active.chunk_entity != chunk);
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
        components: vec![],
    });

    info!("Chunk {chunk_id} was replaced with {chunk_asset:?}");
}

fn replace_chunk_asset_on_contact_with_sensor(
    mut commands: Commands,
    mut player_chunk: ResMut<ActivePlayerChunks>,
    mut chunk_query: Query<(&mut Chunk, &ChunkId)>,
    sensors_query: Query<(
        &ReplaceAssetSensorChunk,
        &ReplaceAssetSensorChunkHandle,
        &ChunkId,
    )>,
) {
    for active in player_chunk.0.iter_mut() {
        match active.asset_sensor_activated {
            true => continue,
            false => active.asset_sensor_activated = true,
        }
        let Ok((sensor, sensor_handle, ChunkId(chunk_id))) = sensors_query.get(active.chunk_entity)
        else {
            continue;
        };

        info!("Player triggered chunk asset replacement by entering sensor chunk {chunk_id}");

        commands.trigger(ReplaceChunkAsset(sensor.chunk, sensor_handle.asset.clone()));
        let mut chunk_cmds = commands.entity(active.chunk_entity);
        match sensor.invert_after_swap {
            true => {
                let mut new_sensor = sensor.clone();
                std::mem::swap(
                    &mut chunk_query
                        .iter_mut()
                        .find(|(_, ChunkId(id))| *id == new_sensor.chunk.0)
                        .unwrap()
                        .0
                        .descriptor_name,
                    &mut new_sensor.descriptor,
                );
                // Trigger on_insert hook to update the asset handle on [`ReplaceAssetSensorChunkHandle`]
                chunk_cmds.insert(new_sensor);
            }
            false => {
                chunk_cmds
                    .remove::<ReplaceAssetSensorChunk>()
                    .remove::<ReplaceAssetSensorChunkHandle>();
            }
        }
    }
}

fn move_chunk_on_contact_with_sensor(
    mut commands: Commands,
    mut player_chunk: ResMut<ActivePlayerChunks>,
    sensors_query: Query<(&MoveChunkSensorChunk, &ChunkId)>,
) {
    for active in player_chunk.0.iter_mut() {
        match active.move_sensor_activated {
            true => continue,
            false => active.move_sensor_activated = true,
        }
        let Ok((sensor, ChunkId(chunk_id))) = sensors_query.get(active.chunk_entity) else {
            continue;
        };

        info!("Player triggered chunk move by entering sensor chunk {chunk_id}");

        commands.trigger(MoveChunk(sensor.chunk, sensor.x, sensor.z));
        commands
            .entity(active.chunk_entity)
            .remove::<MoveChunkSensorChunk>();
    }
}

fn on_move_chunk(
    event: On<MoveChunk>,
    mut chunk_transform_query: Query<(&ChunkId, &mut Transform)>,
) {
    let MoveChunk(ChunkId(chunk_id), x, z) = *event;

    for (ChunkId(id), mut transform) in &mut chunk_transform_query {
        if *id == chunk_id {
            transform.translation.x = x as f32 * CHUNK_SIZE;
            transform.translation.z = z as f32 * CHUNK_SIZE;
            info!("Moved chunk {chunk_id} to grid position ({x}, {z})");
            return;
        }
    }
}

fn swap_chunks_on_contact_with_sensor(
    mut commands: Commands,
    mut player_chunk: ResMut<ActivePlayerChunks>,
    sensors_query: Query<(&SwapSensorChunk, &ChunkId)>,
) {
    for active in player_chunk.0.iter_mut() {
        match active.swap_sensor_activated {
            true => continue,
            false => active.swap_sensor_activated = true,
        }
        let Ok((
            SwapSensorChunk {
                chunk_a,
                chunk_b,
                preserve_after_swap,
            },
            ChunkId(chunk_id),
        )) = sensors_query.get(active.chunk_entity)
        else {
            continue;
        };

        info!("Player triggered chunk swaps by entering sensor chunk {chunk_id}");

        commands.trigger(SwapChunks(*chunk_a, *chunk_b));
        if !preserve_after_swap {
            commands
                .entity(active.chunk_entity)
                .remove::<SwapSensorChunk>();
        }
    }
}

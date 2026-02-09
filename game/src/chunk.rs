use crate::character_controller::GameLayer;
use crate::prelude::*;

#[derive(Default, Component, Reflect)]
#[require(Transform, Visibility, LevelComponent)]
#[reflect(Component)]
pub struct Chunk;

#[derive(Component, Reflect)]
#[require(Chunk)]
#[reflect(Component)]
pub struct ReplaceableChunk;

#[derive(Debug, Event)]
pub struct SwapChunks(Entity, Entity);

#[derive(Component, Reflect)]
#[require(LevelComponent)]
#[reflect(Component)]
pub struct ChunkCullingEntity;

#[derive(Component)]
#[require(Chunk)]
pub struct ChunkFocused;

#[derive(Resource, Default)]
pub struct LastFocusedChunk(pub Option<Entity>);

// /// the chunk the player is currently in
// #[derive(Resource, Default)]
// pub struct PlayerChunk(pub Option<Entity>);

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastFocusedChunk>()
            // .init_resource::<PlayerChunk>()
            .add_observer(on_swap_chunks)
            .add_systems(Update, print_started_collisions)
            .add_systems(
                Update,
                (update_chunk_focus, swap_chunks_on_chunk_focus).chain(),
            );
    }
}

fn print_started_collisions(mut collision_reader: MessageReader<CollisionStart>) {
    for event in collision_reader.read() {
        info!(
            "{} and {} started colliding",
            event.collider1, event.collider2
        );
    }
}

fn update_chunk_focus(
    mut commands: Commands,
    camera_transform: Single<&GlobalTransform, With<Camera>>,
    spatial_query: SpatialQuery,
    culling_entities: Query<&ChildOf, With<ChunkCullingEntity>>,
    replaceable_chunks: Query<Entity, With<ReplaceableChunk>>,
    focused_chunks: Query<Entity, With<ChunkFocused>>,
    mut last_focused: ResMut<LastFocusedChunk>,
) {
    let ray_pos = camera_transform.translation();
    let ray_dir = camera_transform.forward();

    if let Some(hit) = spatial_query.cast_ray(
        ray_pos,
        ray_dir,
        100.0,
        true,
        &SpatialQueryFilter::default().with_mask(GameLayer::default()),
    ) && let Ok(ChildOf(chunk)) = culling_entities.get(hit.entity)
        && replaceable_chunks.contains(*chunk)
    {
        // If not already focused
        if !focused_chunks.contains(*chunk) {
            commands.entity(*chunk).insert(ChunkFocused);
            info!("Chunk {:?} gained focus", *chunk);
        }
        return;
    }

    // If no hit or not a culling entity, clear focus
    for old_focused in focused_chunks.iter() {
        commands.entity(old_focused).remove::<ChunkFocused>();
        info!("Chunk {:?} lost focus", old_focused);
        last_focused.0 = Some(old_focused);
    }
}

fn on_swap_chunks(event: On<SwapChunks>, mut chunk_query: Query<&mut Transform, With<Chunk>>) {
    let SwapChunks(chunk_a, chunk_b) = *event;

    let Ok([mut transform_a, mut transform_b]) = chunk_query.get_many_mut([chunk_a, chunk_b])
    else {
        return;
    };

    std::mem::swap(&mut transform_a.translation, &mut transform_b.translation);
    info!("Swapped chunk {:?} with {:?}", chunk_a, chunk_b);
}

fn swap_chunks_on_chunk_focus(
    mut commands: Commands,
    mut last_focused: ResMut<LastFocusedChunk>,
    focused_chunks: Query<Entity, Added<ChunkFocused>>,
) {
    for new_focused in focused_chunks.iter() {
        if let Some(last_entity) = last_focused.0
            && last_entity != new_focused
        {
            commands.trigger(SwapChunks(last_entity, new_focused));
        }

        last_focused.0 = None;
    }
}

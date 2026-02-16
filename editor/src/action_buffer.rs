use std::collections::BTreeMap;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SelectedLevel>();
    app.add_systems(OnEnter(Screen::Editor), insert_egui_buffer);
    app.add_systems(
        PreUpdate,
        sync_layout_buffer.run_if(in_state(Screen::Editor)),
    );
}

#[derive(Resource, Default, Debug)]
pub struct SelectedLevel(pub GameLevel);

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EguiActionBuffer {
    pub new_element_name: String,
    pub new_descriptor_name: String,
    pub new_layout_id: String,
    pub new_layout_pos: (String, String),
    /// Count the hashmap inserts to prevent egui salt conflicts
    pub layout_push_counter: usize,
    pub layout_buffer: BTreeMap<u32, ((String, String), Handle<ChunkDescriptor>, Vec<ChunkMarker>)>,
}

fn layout_buffer_from_chunks(
    chunks: &BTreeMap<u32, ChunkEntry>,
) -> BTreeMap<u32, ((String, String), Handle<ChunkDescriptor>, Vec<ChunkMarker>)> {
    chunks
        .iter()
        .map(|(id, entry)| {
            (
                *id,
                (
                    (entry.grid_pos.0.to_string(), entry.grid_pos.1.to_string()),
                    entry.descriptor.clone(),
                    entry.components.clone(),
                ),
            )
        })
        .collect()
}

fn insert_egui_buffer(
    mut commands: Commands,
    layouts: Res<Assets<ChunkLayout>>,
    stash: Res<ChunkAssetStash>,
    selected: Res<SelectedLevel>,
) {
    let layout_handle = stash.layout(&selected.0);
    let layout = layouts.get(layout_handle).unwrap();
    commands.insert_resource(EguiActionBuffer {
        layout_buffer: layout_buffer_from_chunks(&layout.chunks),
        ..Default::default()
    });
}

/// Reload the action buffer when the selected level changes.
pub fn reload_layout_buffer(chunks: &BTreeMap<u32, ChunkEntry>, buffer: &mut EguiActionBuffer) {
    buffer.layout_buffer = layout_buffer_from_chunks(chunks);
}

fn sync_layout_buffer(
    mut layouts: ResMut<Assets<ChunkLayout>>,
    buffer: Res<EguiActionBuffer>,
    stash: Res<ChunkAssetStash>,
    selected: Res<SelectedLevel>,
) {
    let layout_buffer = buffer.layout_buffer.iter().fold(
        BTreeMap::new(),
        |mut acc, (id, ((x, z), handle, markers))| {
            let Ok(x) = x.parse() else { return acc };
            let Ok(z) = z.parse() else { return acc };
            acc.insert(
                *id,
                ChunkEntry {
                    grid_pos: (x, z),
                    descriptor: handle.clone(),
                    components: markers.clone(),
                },
            );
            acc
        },
    );

    let layout_handle = stash.layout(&selected.0);
    if let Some(layout) = layouts.get_mut(layout_handle) {
        layout.chunks = layout_buffer;
    }
}

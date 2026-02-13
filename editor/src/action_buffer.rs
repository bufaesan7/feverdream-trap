use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Editor), insert_egui_buffer);
    app.add_systems(
        PreUpdate,
        sync_layout_buffer.run_if(in_state(Screen::Editor)),
    );
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EguiActionBuffer {
    pub new_element_name: String,
    pub new_descriptor_name: String,
    pub new_layout_id: String,
    pub new_layout_pos: (String, String),
    /// Count the hashmap inserts to prevent egui salt conflicts
    pub layout_push_counter: usize,
    pub layout_buffer: Vec<((String, String), Handle<ChunkDescriptor>, Vec<ChunkMarker>)>,
}

fn insert_egui_buffer(mut commands: Commands, layout: Res<Assets<ChunkLayout>>) {
    let layout = &layout.iter().next().unwrap().1.chunks;
    commands.insert_resource(EguiActionBuffer {
        layout_buffer: layout
            .iter()
            .map(
                |ChunkEntry {
                     grid_pos: (x, y),
                     descriptor,
                     components,
                 }| {
                    (
                        (x.to_string(), y.to_string()),
                        descriptor.clone(),
                        components.clone(),
                    )
                },
            )
            .collect(),
        ..Default::default()
    });
}

fn sync_layout_buffer(mut layout: ResMut<Assets<ChunkLayout>>, buffer: Res<EguiActionBuffer>) {
    let layout_buffer =
        buffer
            .layout_buffer
            .iter()
            .fold(vec![], |mut acc, ((x, y), handle, markers)| {
                let Ok(x) = x.parse() else { return acc };
                let Ok(y) = y.parse() else { return acc };
                acc.push(ChunkEntry {
                    grid_pos: (x, y),
                    descriptor: handle.clone(),
                    components: markers.clone(),
                });
                acc
            });

    layout.iter_mut().next().unwrap().1.chunks = layout_buffer.into_iter().collect();
}

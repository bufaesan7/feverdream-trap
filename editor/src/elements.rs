use std::path::PathBuf;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EguiActionBuffer>();
    app.load_resource::<AssetHandleStash>();
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EguiActionBuffer {
    pub new_element_name: String,
    pub new_descriptor_name: String,
}

#[derive(Asset, Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
/// Used to keep the assets loaded
pub struct AssetHandleStash {
    pub elements: Vec<Handle<ChunkElement>>,
    pub descriptors: Vec<Handle<ChunkDescriptor>>,
}

impl FromWorld for AssetHandleStash {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let chunk_asset_path = PathBuf::from("assets/".to_string() + ChunkDescriptorAsset::PATH);

        let mut elements = vec![];
        let mut descriptors = vec![];

        // Recursively walk the directory
        visit_dirs(&chunk_asset_path, &mut |entry| {
            // Get the relative path from assets/chunks/
            let full_path = entry.path();
            let relative_path = full_path
                .strip_prefix("assets/")
                .unwrap_or(&full_path)
                .to_path_buf();

            // Check file extension
            let str_path = relative_path.to_str().unwrap();
            if str_path.ends_with(ChunkElementAsset::EXTENSION) {
                elements.push(asset_server.load(relative_path));
            } else if str_path.ends_with(ChunkDescriptorAsset::EXTENSION) {
                descriptors.push(asset_server.load(relative_path));
            }
        });

        debug!(
            "Loaded {} chunk elements and {} chunk descriptors",
            elements.len(),
            descriptors.len()
        );

        Self {
            elements,
            descriptors,
        }
    }
}

fn visit_dirs(dir: &PathBuf, cb: &mut dyn FnMut(&std::fs::DirEntry)) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recursively visit subdirectories
                visit_dirs(&path, cb);
            } else {
                // Process file
                cb(&entry);
            }
        }
    }
}

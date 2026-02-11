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

        let mut chunk_asset_path = PathBuf::from("assets");
        chunk_asset_path.push("chunks");
        let chunk_assets = std::fs::read_dir(chunk_asset_path).unwrap();

        let mut elements = vec![];
        let mut descriptors = vec![];
        for entry in chunk_assets.flatten() {
            let mut path = PathBuf::from("chunks");
            path.push(
                entry
                    .path()
                    .components()
                    .next_back()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
            );
            // Needed because Path::ends_with compares the entire last segment
            let str_path = path.to_str().unwrap();
            if str_path.ends_with(ChunkElementAsset::EXTENSION) {
                elements.push(asset_server.load(path));
            } else if str_path.ends_with(ChunkDescriptorAsset::EXTENSION) {
                descriptors.push(asset_server.load(path));
            }
        }
        Self {
            elements,
            descriptors,
        }
    }
}

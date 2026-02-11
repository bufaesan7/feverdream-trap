use std::path::PathBuf;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AssetHandleStash>();
    app.init_resource::<EguiActionBuffer>();
    app.load_resource::<ChunkElementStash>();
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EguiActionBuffer {
    pub new_asset_name: String,
}

#[derive(Default, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct AssetHandleStash {
    pub elements: Vec<Handle<ChunkElement>>,
}

#[derive(Asset, Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct ChunkElementStash(pub Vec<Handle<ChunkElement>>);

impl FromWorld for ChunkElementStash {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let mut chunk_asset_path = PathBuf::from("assets");
        chunk_asset_path.push("chunks");
        let chunk_assets = std::fs::read_dir(chunk_asset_path).unwrap();

        let mut elements = vec![];
        for entry in chunk_assets.flatten() {
            if entry
                .path()
                .to_str()
                .unwrap()
                .ends_with(ChunkElementAsset::EXTENSION)
            {
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
                elements.push(asset_server.load(path));
            }
        }
        Self(elements)
    }
}

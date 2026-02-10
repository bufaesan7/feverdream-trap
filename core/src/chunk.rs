use serde::Deserialize;

use crate::{
    asset_loader::{RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<ChunkLayout>()
        .register_asset_loader(RonAssetLoader::<ChunkElementAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkDescriptorAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkLayoutAsset>::new())
        .load_resource::<ChunkLayoutStorage>();
}

pub const CHUNK_SIZE: Vec2 = Vec2 { x: 16., y: 16. };

#[derive(Debug, Deserialize)]
pub enum ChunkElementShapeAsset {
    Cube,
    Sphere,
    Gltf { mesh: String },
}

#[derive(Debug)]
pub enum ChunkElementShape {
    Cube,
    Sphere,
    Gltf { mesh: Handle<Gltf> },
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct ChunkElementAsset {
    pub name: String,
    pub transform: Transform,
    pub shape: ChunkElementShapeAsset,
}

#[derive(Asset, TypePath, Debug)]
pub struct ChunkElement {
    pub name: String,
    pub transform: Transform,
    pub shape: ChunkElementShape,
}

impl RonAsset for ChunkElementAsset {
    type Asset = ChunkElement;
    const EXTENSION: &str = ".chunk.element";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let shape = match self.shape {
            ChunkElementShapeAsset::Cube => ChunkElementShape::Cube,
            ChunkElementShapeAsset::Sphere => ChunkElementShape::Sphere,
            ChunkElementShapeAsset::Gltf { mesh } => ChunkElementShape::Gltf {
                mesh: context.load(mesh),
            },
        };
        ChunkElement {
            name: self.name,
            transform: self.transform,
            shape,
        }
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct ChunkDescriptorAsset {
    pub elements: Vec<String>,
}

#[derive(Asset, TypePath, Debug)]
pub struct ChunkDescriptor {
    #[dependency]
    pub elements: Vec<Handle<ChunkElement>>,
}

impl RonAsset for ChunkDescriptorAsset {
    type Asset = ChunkDescriptor;
    const EXTENSION: &str = ".chunk";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let elements = self
            .elements
            .into_iter()
            .map(|path| context.load(path + ChunkElementAsset::EXTENSION))
            .collect();
        ChunkDescriptor { elements }
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct ChunkLayoutAsset {
    pub grid: HashMap<(i32, i32), String>,
}

#[derive(Asset, TypePath, Debug)]
pub struct ChunkLayout {
    /// Maps chunk positions (in chunk space, world space is obtained by multiplying by
    /// [`CHUNK_SIZE`]) to [`ChunkDescriptor`]
    pub grid: HashMap<(i32, i32), Handle<ChunkDescriptor>>,
}

impl RonAsset for ChunkLayoutAsset {
    type Asset = ChunkLayout;
    const EXTENSION: &str = ".layout";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let grid = self
            .grid
            .into_iter()
            .map(|(pos, path)| (pos, context.load(path + ChunkDescriptorAsset::EXTENSION)))
            .collect();
        ChunkLayout { grid }
    }
}

#[derive(Resource, Asset, TypePath, Clone)]
pub struct ChunkLayoutStorage {
    pub handle: Handle<ChunkLayout>,
}

impl FromWorld for ChunkLayoutStorage {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ChunkLayoutStorage {
            handle: asset_server.load("chunks/chunk".to_string() + ChunkLayoutAsset::EXTENSION),
        }
    }
}

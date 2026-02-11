use serde::{Deserialize, Serialize};

use crate::{
    asset_loader::{RonAsset, RonAssetLoader},
    prelude::*,
};

use bevy::asset::ReflectAsset;

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<ChunkElement>()
        .init_asset::<ChunkDescriptor>()
        .init_asset::<ChunkLayout>()
        .register_asset_reflect::<ChunkElement>()
        .register_asset_loader(RonAssetLoader::<ChunkElementAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkDescriptorAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkLayoutAsset>::new())
        .load_resource::<ChunkLayoutStorage>();
}

pub const CHUNK_SIZE: Vec2 = Vec2 { x: 16., y: 16. };

#[derive(Debug, Serialize, Deserialize)]
pub enum ChunkElementShapeAsset {
    Cube,
    Sphere,
    Gltf { mesh_path: String },
}

#[derive(Debug, Reflect, Default)]
#[reflect(Default)]
pub enum ChunkElementShape {
    #[default]
    Cube,
    Sphere,
    Gltf {
        mesh_path: String,
        mesh: Handle<Gltf>,
    },
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ChunkElementAsset {
    pub name: String,
    pub transform: Transform,
    pub shape: ChunkElementShapeAsset,
}

#[derive(Asset, Reflect, Debug, Default)]
#[reflect(Asset)]
pub struct ChunkElement {
    pub name: String,
    pub transform: Transform,
    pub shape: ChunkElementShape,
}

impl ChunkElement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl From<&ChunkElement> for ChunkElementAsset {
    fn from(value: &ChunkElement) -> Self {
        Self {
            name: value.name.clone(),
            transform: value.transform,
            shape: match &value.shape {
                ChunkElementShape::Cube => ChunkElementShapeAsset::Cube,
                ChunkElementShape::Sphere => ChunkElementShapeAsset::Sphere,
                ChunkElementShape::Gltf { mesh_path, .. } => ChunkElementShapeAsset::Gltf {
                    mesh_path: mesh_path.clone(),
                },
            },
        }
    }
}

impl RonAsset for ChunkElementAsset {
    type Asset = ChunkElement;
    const EXTENSION: &str = "chunk.element";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let shape = match self.shape {
            ChunkElementShapeAsset::Cube => ChunkElementShape::Cube,
            ChunkElementShapeAsset::Sphere => ChunkElementShape::Sphere,
            ChunkElementShapeAsset::Gltf { mesh_path } => ChunkElementShape::Gltf {
                mesh: context.load(&mesh_path),
                mesh_path,
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
    pub name: String,
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
            .map(|path| context.load(path + "." + ChunkElementAsset::EXTENSION))
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

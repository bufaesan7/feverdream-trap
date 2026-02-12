use std::path::PathBuf;

#[cfg(feature = "dev_native")]
use bevy_inspector_egui::inspector_egui_impls::InspectorEguiImpl;
use serde::{Deserialize, Serialize};

use crate::{
    asset_loader::{RonAsset, RonAssetLoader},
    prelude::*,
};

use bevy::asset::{ReflectAsset, VisitAssetDependencies};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<ChunkElement>()
        .init_asset::<ChunkDescriptor>()
        .init_asset::<ChunkLayout>()
        .register_asset_loader(RonAssetLoader::<ChunkElementAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkDescriptorAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkLayoutAsset>::new())
        .load_resource::<ChunkLayoutStorage>();

    #[cfg(feature = "dev_native")]
    app.register_type_data::<Wrapper<Handle<ChunkElement>>, InspectorEguiImpl>();
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChunkElementShapeAsset {
    Plane,
    Cube,
    Sphere,
    Gltf { mesh_path: String },
}

#[derive(Debug, Reflect, Clone)]
pub enum ChunkElementShape {
    Plane,
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

#[derive(Asset, Reflect, Debug, Clone)]
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
            transform: Transform::default(),
            shape: ChunkElementShape::Cube,
        }
    }
}

impl ChunkElementAsset {
    pub const PATH: &str = "chunks/elements";

    pub fn path(&self) -> PathBuf {
        Self::path_from_name(&self.name)
    }

    pub fn path_from_name(name: &str) -> PathBuf {
        let mut path = PathBuf::from(Self::PATH);
        path.push(name.to_string() + "." + Self::EXTENSION);

        path
    }
}

impl From<&ChunkElement> for ChunkElementAsset {
    fn from(value: &ChunkElement) -> Self {
        Self {
            name: value.name.clone(),
            transform: value.transform,
            shape: match &value.shape {
                ChunkElementShape::Plane => ChunkElementShapeAsset::Plane,
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
            ChunkElementShapeAsset::Plane => ChunkElementShape::Plane,
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

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ChunkDescriptorAsset {
    pub name: String,
    pub elements: Vec<String>,
}

#[derive(Reflect, Debug, Default, Clone, Deref)]
#[reflect(Default)]
/// New type wrapper to allow implementing
/// [`bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive`]
pub struct Wrapper<T: Default>(pub T);

#[derive(Reflect, Debug)]
#[reflect(Asset)]
pub struct ChunkDescriptor {
    pub name: String,
    pub elements: Vec<Wrapper<Handle<ChunkElement>>>,
}

impl ChunkDescriptor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            elements: vec![],
        }
    }
}

impl Asset for ChunkDescriptor {}

impl VisitAssetDependencies for ChunkDescriptor {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        for e in &self.elements {
            visit(e.id().untyped())
        }
    }
}

#[cfg(feature = "dev_native")]
impl bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive
    for Wrapper<Handle<ChunkElement>>
{
    fn ui(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        _options: &dyn std::any::Any,
        _id: bevy_egui::egui::Id,
        env: bevy_inspector_egui::reflect_inspector::InspectorUi<'_, '_>,
    ) -> bool {
        let world = env.context.world.as_mut().unwrap();
        let (element_assets, asset_server) =
            world.get_two_resources_mut::<Assets<ChunkElement>, AssetServer>();
        let element_assets = element_assets.unwrap();
        let asset_server = asset_server.unwrap();
        let selected_name = element_assets
            .get(&self.0)
            .map(|e| e.name.clone())
            .unwrap_or_default();
        ui.push_id(self.0.id(), |ui| {
            let selected = &mut self.0;
            egui::ComboBox::from_label("Pick handle")
                .selected_text(selected_name)
                .show_ui(ui, |ui| {
                    for (index, (id, asset)) in element_assets.iter().enumerate() {
                        ui.push_id(index, |ui| {
                            ui.selectable_value(
                                selected,
                                asset_server.get_id_handle(id).unwrap(),
                                &asset.name,
                            );
                        });
                    }
                });
        });
        false
    }
    fn ui_readonly(
        &self,
        ui: &mut bevy_egui::egui::Ui,
        _options: &dyn std::any::Any,
        _id: bevy_egui::egui::Id,
        _env: bevy_inspector_egui::reflect_inspector::InspectorUi<'_, '_>,
    ) {
        ui.label("Hello ui_readonly");
    }
}

impl ChunkDescriptorAsset {
    pub const PATH: &str = "chunks";

    pub fn path(&self) -> PathBuf {
        Self::path_from_name(&self.name)
    }

    pub fn path_from_name(name: &str) -> PathBuf {
        let mut path = PathBuf::from(Self::PATH);
        path.push(name.to_string() + "." + Self::EXTENSION);

        path
    }
}

impl From<(&ChunkDescriptor, &Assets<ChunkElement>)> for ChunkDescriptorAsset {
    fn from((value, assets): (&ChunkDescriptor, &Assets<ChunkElement>)) -> Self {
        Self {
            name: value.name.clone(),
            elements: value
                .elements
                .iter()
                .filter_map(|handle| assets.get(&handle.0).map(|e| e.name.clone()))
                .collect(),
        }
    }
}

impl RonAsset for ChunkDescriptorAsset {
    type Asset = ChunkDescriptor;
    const EXTENSION: &str = "chunk";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let elements = self
            .elements
            .into_iter()
            .map(|name| Wrapper(context.load(ChunkElementAsset::path_from_name(&name))))
            .collect();

        ChunkDescriptor {
            name: self.name,
            elements,
        }
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
    const EXTENSION: &str = "layout";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let grid = self
            .grid
            .into_iter()
            .map(|(pos, name)| {
                (
                    pos,
                    context.load(ChunkDescriptorAsset::path_from_name(&name)),
                )
            })
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
            handle: asset_server
                .load("chunks/chunk".to_string() + "." + ChunkLayoutAsset::EXTENSION),
        }
    }
}

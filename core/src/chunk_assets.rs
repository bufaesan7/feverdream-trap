use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[cfg(feature = "dev_native")]
use bevy_inspector_egui::inspector_egui_impls::InspectorEguiImpl;
use serde::{Deserialize, Serialize};

use crate::{
    asset_loader::{RonAsset, RonAssetLoader},
    prelude::*,
};

use bevy::asset::{ReflectAsset, VisitAssetDependencies};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Reflect)]
pub enum GameLevel {
    #[default]
    Demo,
    Level1,
}

impl GameLevel {
    pub const ALL: &[GameLevel] = &[GameLevel::Demo, GameLevel::Level1];

    pub fn path(&self) -> PathBuf {
        let name = match self {
            GameLevel::Demo => "demo",
            GameLevel::Level1 => "level_1",
        };
        PathBuf::from("levels").join(format!("{name}.layout"))
    }

    pub fn next(&self) -> Option<GameLevel> {
        let idx = Self::ALL.iter().position(|l| l == self).unwrap();
        Self::ALL.get(idx + 1).copied()
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ChunkMarker>()
        .init_asset::<ChunkElement>()
        .init_asset::<ChunkDescriptor>()
        .init_asset::<ChunkLayout>()
        .register_asset_loader(RonAssetLoader::<ChunkElementAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkDescriptorAsset>::new())
        .register_asset_loader(RonAssetLoader::<ChunkLayoutAsset>::new())
        .load_resource::<ChunkAssetStash>();

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

#[derive(Clone, Reflect, Debug)]
#[reflect(Default)]
/// TODO: remove this and add avian collider-from-mesh feature instead
pub struct DefaultWrap(pub ColliderConstructor);
impl Default for DefaultWrap {
    fn default() -> Self {
        Self(ColliderConstructor::Cuboid {
            x_length: 1.,
            y_length: 1.,
            z_length: 1.,
        })
    }
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ChunkElementAsset {
    pub name: String,
    pub transform: Transform,
    pub shape: ChunkElementShapeAsset,
    pub color: Color,
    /// Has no effect on [`ChunkElementShapeAsset::Gltf`]
    pub has_collider: bool,
}

#[derive(Asset, Reflect, Debug, Clone)]
#[reflect(Asset)]
pub struct ChunkElement {
    pub name: String,
    pub transform: Transform,
    pub shape: ChunkElementShape,
    pub color: Color,
    pub has_collider: bool,
}

impl ChunkElement {
    pub fn new(name: String) -> Self {
        Self {
            name,
            transform: Transform::default(),
            shape: ChunkElementShape::Cube,
            color: Color::WHITE,
            has_collider: true,
        }
    }
}

impl ChunkElementAsset {
    pub const PATH: [&str; 2] = ["chunks", "elements"];

    pub fn path(&self) -> PathBuf {
        Self::path_from_name(&self.name)
    }

    fn path_from_name(name: &str) -> PathBuf {
        let mut path = PathBuf::from_iter(Self::PATH);
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
            color: value.color,
            has_collider: value.has_collider,
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
            color: self.color,
            has_collider: self.has_collider,
        }
    }
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ChunkDescriptorAsset {
    pub name: String,
    pub elements: Vec<String>,
}

#[derive(Reflect, Debug, Default, Clone)]
#[reflect(Default)]
/// New type wrapper to allow implementing
/// [`bevy_inspector_egui::inspector_egui_impls::InspectorPrimitive`]
/// `Self.1` is an ID needed by egui to distinguish picker widgets with the same value
pub struct Wrapper<T: Default>(pub T, #[cfg(feature = "dev_native")] pub usize);

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

    pub fn get_elements(&self, chunk_elements: &Assets<ChunkElement>) -> Option<Vec<ChunkElement>> {
        self.elements
            .iter()
            .map(|element| chunk_elements.get(&element.0).cloned())
            .collect()
    }
}

impl Asset for ChunkDescriptor {}

impl VisitAssetDependencies for ChunkDescriptor {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        for e in &self.elements {
            visit(e.0.id().untyped())
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
        #[cfg(feature = "dev_native")]
        let elements = self
            .elements
            .into_iter()
            .enumerate()
            .map(|(id, name)| Wrapper(context.load(ChunkElementAsset::path_from_name(&name)), id))
            .collect();
        #[cfg(not(feature = "dev_native"))]
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

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Default)]
pub enum ChunkMarker {
    PlayerSpawn(Transform),
    Light(ChunkLight),
    /// Mark this chunk as a sensor chunk, that will cause two other chunks to be swapped when this
    /// one is entered by the player
    SwapSensor(SwapSensorChunk),
    /// Mark this chunk as a sensor chunk, that will cause one chunk to be replaced with a
    /// not-yet-loaded chunk asset when this one is entered by the player
    ReplaceAssetSensor(ReplaceAssetSensorChunk),
    /// Mark this chunk as a sensor chunk, that will move a given chunk to a given position
    /// when this one is entered by the player
    MoveChunkSensor(MoveChunkSensorChunk),
}

impl Default for ChunkMarker {
    fn default() -> Self {
        ChunkMarker::PlayerSpawn(Transform::default())
    }
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ChunkEntryAsset {
    pub grid_pos: (i32, i32),
    pub descriptor: String,
    #[serde(default)]
    pub components: Vec<ChunkMarker>,
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct ChunkLayoutAsset {
    pub chunks: BTreeMap<u32, ChunkEntryAsset>,
}

#[derive(Reflect, Debug, Clone)]
pub struct ChunkEntry {
    pub grid_pos: (i32, i32),
    pub descriptor: Handle<ChunkDescriptor>,
    pub components: Vec<ChunkMarker>,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct ChunkLayout {
    pub chunks: BTreeMap<u32, ChunkEntry>,
}

impl From<(&ChunkLayout, &Assets<ChunkDescriptor>)> for ChunkLayoutAsset {
    fn from((value, descriptors): (&ChunkLayout, &Assets<ChunkDescriptor>)) -> Self {
        let chunks = value
            .chunks
            .iter()
            .map(|(id, entry)| {
                (
                    *id,
                    ChunkEntryAsset {
                        grid_pos: entry.grid_pos,
                        descriptor: descriptors.get(&entry.descriptor).unwrap().name.clone(),
                        components: entry.components.clone(),
                    },
                )
            })
            .collect();
        Self { chunks }
    }
}

impl ChunkLayoutAsset {
    pub fn path_for_level(level: &GameLevel) -> PathBuf {
        level.path()
    }
}

impl RonAsset for ChunkLayoutAsset {
    type Asset = ChunkLayout;
    const EXTENSION: &str = "layout";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        let chunks = self
            .chunks
            .into_iter()
            .map(|(id, entry)| {
                let descriptor =
                    context.load(ChunkDescriptorAsset::path_from_name(&entry.descriptor));
                (
                    id,
                    ChunkEntry {
                        grid_pos: entry.grid_pos,
                        descriptor,
                        components: entry.components,
                    },
                )
            })
            .collect();
        ChunkLayout { chunks }
    }
}

#[derive(Asset, TypePath, Resource, Debug, Clone)]
pub struct ChunkAssetStash {
    #[dependency]
    pub elements: Vec<Handle<ChunkElement>>,
    #[dependency]
    pub descriptors: Vec<Handle<ChunkDescriptor>>,
    #[dependency]
    pub layout_handles: Vec<Handle<ChunkLayout>>,
    /// Parallel to `layout_handles` — maps each handle to a `GameLevel`.
    pub layout_levels: Vec<GameLevel>,
}

impl ChunkAssetStash {
    pub fn layout(&self, level: &GameLevel) -> &Handle<ChunkLayout> {
        let idx = self
            .layout_levels
            .iter()
            .position(|l| l == level)
            .unwrap_or_else(|| panic!("No layout loaded for level {level:?}"));
        &self.layout_handles[idx]
    }
}

impl FromWorld for ChunkAssetStash {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let chunk_asset_path = Path::new("assets").join(ChunkDescriptorAsset::PATH);

        let mut elements = vec![];
        let mut descriptors = vec![];

        visit_dirs(&chunk_asset_path, &mut |entry| {
            let full_path = entry.path();
            let relative_path = full_path
                .strip_prefix("assets")
                .unwrap_or(&full_path)
                .to_path_buf();

            let str_path = relative_path.to_str().unwrap();
            if str_path.ends_with(ChunkElementAsset::EXTENSION) {
                elements.push(asset_server.load(relative_path));
            } else if str_path.ends_with(ChunkDescriptorAsset::EXTENSION) {
                descriptors.push(asset_server.load(relative_path));
            }
        });

        let layout_levels: Vec<GameLevel> = GameLevel::ALL.to_vec();
        let layout_handles: Vec<Handle<ChunkLayout>> = layout_levels
            .iter()
            .map(|level| asset_server.load(level.path()))
            .collect();

        debug!(
            "Loaded {} chunk elements, {} chunk descriptors, and {} layouts",
            elements.len(),
            descriptors.len(),
            layout_handles.len()
        );

        Self {
            elements,
            descriptors,
            layout_handles,
            layout_levels,
        }
    }
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&std::fs::DirEntry)) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb);
            } else {
                cb(&entry);
            }
        }
    }
}

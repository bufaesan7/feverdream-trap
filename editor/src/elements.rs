use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Editor), insert_element_stash);
}

fn insert_element_stash(world: &World, mut commands: Commands) {
    commands.insert_resource(ChunkElementStash::new(world));
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct ChunkElementStash(pub Vec<ChunkElementAsset>);

impl ChunkElementStash {
    pub fn new(world: &World) -> Self {
        let assets = world.resource::<Assets<ChunkElement>>();
        let elements = assets
            .iter()
            .map(|(_, element)| ChunkElementAsset {
                name: element.name.clone(),
                transform: element.transform,
                shape: match &element.shape {
                    ChunkElementShape::Cube => ChunkElementShapeAsset::Cube,
                    ChunkElementShape::Sphere => ChunkElementShapeAsset::Sphere,
                    ChunkElementShape::Gltf { mesh_path, .. } => ChunkElementShapeAsset::Gltf {
                        mesh_path: mesh_path.clone(),
                    },
                },
            })
            .collect();
        Self(elements)
    }
}

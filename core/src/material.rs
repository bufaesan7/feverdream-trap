use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct CoreMaterialPlugin;

impl Plugin for CoreMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighlightStorageBuffer>()
            .add_plugins(MaterialPlugin::<
                ExtendedMaterial<StandardMaterial, HighlightExtension>,
            >::default());
    }
}

const SHADER_ASSET_PATH: &str = "shaders/blend.wgsl";

/// This resource holds the common color storage buffer for highlighting
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct HighlightStorageBuffer(pub Handle<ShaderStorageBuffer>);

impl FromWorld for HighlightStorageBuffer {
    fn from_world(world: &mut World) -> Self {
        let mut buffers = world
            .get_resource_mut::<Assets<ShaderStorageBuffer>>()
            .unwrap();
        let color_data: Vec<[f32; 4]> = vec![[1.0, 1.0, 1.0, 1.0], [0.9, 0.9, 0.9, 0.9]];

        Self(buffers.add(ShaderStorageBuffer::from(color_data)))
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HighlightExtension {
    #[storage(100, read_only)]
    pub colors: Handle<ShaderStorageBuffer>,
}

impl MaterialExtension for HighlightExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

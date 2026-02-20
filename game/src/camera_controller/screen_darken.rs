use bevy::{
    core_pipeline::{
        core_3d::graph::Node3d,
        fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    },
    render::{
        extract_component::ExtractComponent, render_graph::RenderLabel as _,
        render_resource::ShaderType,
    },
    shader::ShaderRef,
};

use crate::{
    camera_controller::{CameraMarker, CameraStatusEffects, status_effects::CameraEffect},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FullscreenMaterialPlugin::<ScreenDarkenEffect>::default());
}

// See
// <https://github.com/bevyengine/bevy/blob/681751647a588b5500ad0b9fe5cd2690c10b1003/examples/shader_advanced/fullscreen_material.rs>

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default, Reflect)]
#[reflect(Component)]
pub struct ScreenDarkenEffect {
    intensity: f32,
    time: f32,
    _web_gl_padding_a: f32,
    _web_gl_padding_b: f32,
}

impl FullscreenMaterial for ScreenDarkenEffect {
    fn fragment_shader() -> ShaderRef {
        "shaders/screen_darken_effect.wgsl".into()
    }

    fn node_edges() -> Vec<bevy::render::render_graph::InternedRenderLabel> {
        vec![
            Node3d::Tonemapping.intern(),
            // The label is automatically generated from the name of the struct
            Self::node_label().intern(),
            Node3d::EndMainPassPostProcessing.intern(),
        ]
    }
}

pub fn apply_screen_darken_intensity(
    time: Res<Time>,
    effects: Res<CameraStatusEffects>,
    mut effect: Single<&mut ScreenDarkenEffect, With<CameraMarker>>,
) {
    effect.intensity = effects.effects[&CameraEffect::ScreenDarken].intensity;
    effect.time = time.elapsed_secs();
}

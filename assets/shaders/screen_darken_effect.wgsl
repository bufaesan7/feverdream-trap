#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct FullScreenEffect {
    intensity: f32,
    time: f32,
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec2<f32>
}

@group(0) @binding(2) var<uniform> settings: FullScreenEffect;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let t = sin(settings.time * settings.intensity) * 0.5 + 0.5;
    let uv = (in.uv - vec2f(0.5, 0.5)) * 2.;
    let lens_mask = sqrt(2.) - length(uv);
    let intensity = max(settings.intensity * 0.5, 0.5);

    let dark_scale = (1 - t * settings.intensity * 0.5) * lens_mask * intensity;
    return vec4<f32>(
        (textureSample(screen_texture, texture_sampler, in.uv) * dark_scale).rgb,
        1.0
    );
}

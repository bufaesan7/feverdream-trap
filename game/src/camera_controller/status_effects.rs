use bevy::{
    asset::RenderAssetUsages,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    post_process::effect_stack::ChromaticAberration,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    camera_controller::{CameraMarker, screen_darken::apply_screen_darken_intensity},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            intensify_effects,
            (
                apply_chromatic_aberration_intensity,
                apply_screen_darken_intensity,
            )
                .after(intensify_effects),
        )
            .in_set(PausableSystems),
    );
}

#[derive(Reflect, Debug, PartialEq, Eq, Hash)]
pub enum CameraEffect {
    ChromaticAbberation,
    ScreenDarken,
}

#[derive(Reflect, Debug)]
pub struct StatusEffect {
    /// Current effect intensity
    pub intensity: f32,
    /// Increase of `self.intensity` per second
    pub intensification_speed: f32,
}

impl Default for StatusEffect {
    fn default() -> Self {
        Self {
            intensity: 0.,
            intensification_speed: 0.01,
        }
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
/// The contained [`StatusEffect`] values can be controlled by drugs
pub struct CameraStatusEffects {
    pub effect_intensities: HashMap<CameraEffect, StatusEffect>,
}

impl Default for CameraStatusEffects {
    fn default() -> Self {
        Self {
            effect_intensities: HashMap::from_iter([
                (CameraEffect::ChromaticAbberation, Default::default()),
                (CameraEffect::ScreenDarken, Default::default()),
            ]),
        }
    }
}

pub(super) fn add_camera_effects<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
    let Some(mut images) = world.get_resource_mut::<Assets<Image>>() else {
        return;
    };

    // RGBA, used for [`ChromaticAbberation`]
    let img_data = vec![
        //
        255, 0, 0, 255, //
        //
        0, 255, 0, 255, //
        //
        0, 0, 255, 255,
    ];
    let color_lut = Image::new(
        Extent3d {
            width: 3,
            ..Default::default()
        },
        TextureDimension::D2,
        img_data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    let color_lut = Some(images.add(color_lut));

    let mut cmds = world.commands();
    let mut camera_cmds = cmds.entity(hook.entity);

    camera_cmds.insert(ChromaticAberration {
        color_lut,
        intensity: 0.,
        max_samples: 4,
    });
}

fn intensify_effects(time: Res<Time>, mut effects: ResMut<CameraStatusEffects>) {
    let delta = time.delta_secs();
    for StatusEffect {
        intensity,
        intensification_speed,
    } in effects.effect_intensities.values_mut()
    {
        *intensity += delta * *intensification_speed;
    }
}

fn apply_chromatic_aberration_intensity(
    effects: Res<CameraStatusEffects>,
    mut effect: Single<&mut ChromaticAberration, With<CameraMarker>>,
) {
    effect.intensity = effects.effect_intensities[&CameraEffect::ChromaticAbberation].intensity;
}

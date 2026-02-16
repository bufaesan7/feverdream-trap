use bevy::{
    asset::RenderAssetUsages,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    post_process::effect_stack::ChromaticAberration,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    camera_controller::{
        CameraMarker,
        drugs::{DrugEffectSet, DrugInteraction},
        screen_darken::apply_screen_darken_intensity,
    },
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

#[derive(Reflect, Debug, PartialEq, Eq, Hash, Default)]
#[reflect(Default)]
pub enum CameraEffect {
    #[default]
    ChromaticAbberation,
    ScreenDarken,
}

#[derive(Reflect, Debug)]
pub struct StatusEffect {
    /// Current effect intensity
    pub intensity: f32,
    /// Increase of `self.intensity` per second
    intensification_speed: f32,
    temporary_intensification_overwrite: Option<(Timer, f32)>,
}

impl Default for StatusEffect {
    fn default() -> Self {
        Self {
            intensity: 0.,
            intensification_speed: 0.01,
            temporary_intensification_overwrite: None,
        }
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
/// The contained [`StatusEffect`] values can be controlled by drugs
pub struct CameraStatusEffects {
    pub effects: HashMap<CameraEffect, StatusEffect>,
}

impl Default for CameraStatusEffects {
    fn default() -> Self {
        Self {
            effects: HashMap::from_iter([
                (CameraEffect::ChromaticAbberation, Default::default()),
                (CameraEffect::ScreenDarken, Default::default()),
            ]),
        }
    }
}

impl CameraStatusEffects {
    pub(super) fn apply_drug_effects(&mut self, drug_effects: &DrugInteraction) {
        for (camera_effect, drug_effects) in drug_effects.iter() {
            let status = self.effects.get_mut(camera_effect).unwrap();
            for effect in drug_effects {
                match effect {
                    DrugEffectSet::Intensity { value } => status.intensity = *value,
                    DrugEffectSet::Intensification { value } => {
                        status.intensification_speed = *value
                    }
                    DrugEffectSet::IntensificationFor {
                        duration_secs,
                        value,
                    } => {
                        status.temporary_intensification_overwrite = Some((
                            Timer::new(Duration::from_secs_f32(*duration_secs), TimerMode::Once),
                            *value,
                        ))
                    }
                }
            }
        }
    }

    /// Add [`ChromaticAberration`] to camera
    pub(super) fn add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        let Some(mut images) = world.get_resource_mut::<Assets<Image>>() else {
            return;
        };

        // RGBA, used for [`ChromaticAberration`]
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
}

fn intensify_effects(time: Res<Time>, mut effects: ResMut<CameraStatusEffects>) {
    let delta = time.delta_secs();
    for StatusEffect {
        intensity,
        intensification_speed,
        temporary_intensification_overwrite,
    } in effects.effects.values_mut()
    {
        *intensity += delta
            * match temporary_intensification_overwrite {
                Some((timer, overwrite)) => {
                    timer.tick(time.delta());
                    let speed = *overwrite;
                    if timer.just_finished() {
                        *temporary_intensification_overwrite = None;
                    }
                    speed
                }
                None => *intensification_speed,
            };
    }
}

fn apply_chromatic_aberration_intensity(
    effects: Res<CameraStatusEffects>,
    mut effect: Single<&mut ChromaticAberration, With<CameraMarker>>,
) {
    effect.intensity = effects.effects[&CameraEffect::ChromaticAbberation].intensity;
}

use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::{
    camera_controller::{CameraStatusEffects, status_effects::CameraEffect},
    interaction::{Interact, Interactable},
    prelude::*,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(DrugInteraction {
    effects: vec![(
        CameraEffect::ChromaticAbberation,
        vec![DrugEffectSet::Intensity { value: 0. }],
    )]
})]
/// [`DrugInteraction`] preset because skein does not support editing collections (yet)
struct DrugClearChromaticAberrationIntensity;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(not(target_arch = "wasm32"), require(DrugInteraction {
    effects: vec![(
        CameraEffect::ScreenDarken,
        vec![DrugEffectSet::Intensity { value: 0. }],
    )]
}))]
/// [`DrugInteraction`] preset because skein does not support editing collections (yet)
struct DrugClearScreenDarkenIntensity;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(DrugInteraction {
    effects: vec![
        (
            CameraEffect::ChromaticAbberation,
            vec![DrugEffectSet::Intensity { value: 0. }],
        ),
        (
            CameraEffect::ChromaticAbberation,
            vec![DrugEffectSet::IntensificationFor { duration_secs: 5., value: 0. }],
        )
    ]
})]
/// [`DrugInteraction`] preset because skein does not support editing collections (yet)
struct DrugDisableChromaticAberrationFiveSecs;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[cfg_attr(not(target_arch = "wasm32"), require(DrugInteraction {
    effects: vec![
        (
            CameraEffect::ScreenDarken,
            vec![DrugEffectSet::Intensity { value: 0. }],
        ),
        (
            CameraEffect::ScreenDarken,
            vec![DrugEffectSet::IntensificationFor { duration_secs: 5., value: 0. }],
        )
    ]
}))]
/// [`DrugInteraction`] preset because skein does not support editing collections (yet)
struct DrugDisableScreenDarkenFiveSecs;

#[derive(Component, Reflect, Debug, Default, Deref)]
#[reflect(Default, Component)]
#[require(Interactable, LevelComponent, RigidBody::Dynamic)]
#[component(on_add)]
pub struct DrugInteraction {
    effects: Vec<(CameraEffect, Vec<DrugEffectSet>)>,
}

impl DrugInteraction {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        world.commands().entity(hook.entity).observe(
            |event: On<Interact>,
             mut commands: Commands,
             query: Query<&DrugInteraction>,
             mut status_effects: ResMut<CameraStatusEffects>| {
                if let Ok(drug_effects) = query.get(event.entity) {
                    status_effects.apply_drug_effects(drug_effects);
                    commands.entity(event.entity).despawn();
                }
            },
        );
    }
}

#[derive(Reflect, Debug)]
#[reflect(Default)]
pub enum DrugEffectSet {
    /// Set the effect intensity to `value`
    Intensity { value: f32 },
    /// Set the effect intensification to `value`
    Intensification { value: f32 },
    /// Set the effect intensification to `value` for `duration_secs` seconds, then set it back to
    /// where it was
    IntensificationFor { duration_secs: f32, value: f32 },
}

impl Default for DrugEffectSet {
    fn default() -> Self {
        Self::Intensity { value: 0. }
    }
}

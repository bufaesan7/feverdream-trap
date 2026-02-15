use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::{
    camera_controller::{CameraStatusEffects, status_effects::CameraEffect},
    interaction::{Interact, Interactable},
    prelude::*,
};

#[derive(Component, Reflect, Debug, Default, Deref)]
#[reflect(Default, Component)]
#[require(Interactable, LevelComponent)]
#[component(on_add)]
pub struct DrugInteraction {
    effects: HashMap<CameraEffect, Vec<DrugEffectSet>>,
}

impl DrugInteraction {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        world.commands().entity(hook.entity).observe(
            |event: On<Interact>,
             query: Query<&DrugInteraction>,
             mut status_effects: ResMut<CameraStatusEffects>| {
                if let Ok(drug_effects) = query.get(event.entity) {
                    status_effects.apply_drug_effects(drug_effects);
                }
            },
        );
    }
}

#[derive(Reflect, Debug)]
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

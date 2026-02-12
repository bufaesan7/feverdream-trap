mod focus;
mod interactions;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshTag,
};

#[cfg(feature = "dev")]
use crate::interaction::interactions::DebugInteraction;
use crate::{character_controller::GameLayer, interaction::focus::FocusTarget, prelude::*};

pub use focus::*;
pub use interactions::*;

const INTERACTION_DISTANCE: f32 = 10.0;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(focus::plugin).add_systems(
        Update,
        interact.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
}

/// Indicates whether an entity can be interacted with
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[require(MeshTag)] // This is used for highlighting; see focus
#[component(on_add)]
pub struct Interactable;

impl Interactable {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).insert((
            CollisionLayers::new(GameLayer::Interactable, LayerMask::ALL), // This is used for finding focus target; see focus
            #[cfg(feature = "dev")]
            DebugInteraction,
        ));
    }
}

#[derive(Debug, EntityEvent, Reflect)]
#[reflect(Event)]
pub struct Interact {
    pub entity: Entity,
}

impl From<Entity> for Interact {
    fn from(entity: Entity) -> Self {
        Interact { entity }
    }
}

fn interact(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    focus_targets: Query<Entity, With<FocusTarget>>,
) {
    for entity in &focus_targets {
        if mouse.just_pressed(MouseButton::Left) {
            commands.entity(entity).trigger(Interact::from);
        }
    }
}

mod focus;
mod interactions;

use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::{
    interaction::{
        focus::FocusTarget,
        interactions::{register_component_hooks, register_required_components},
    },
    prelude::*,
};

const INTERACTION_DISTANCE: f32 = 5.0;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Fuse>()
        .add_plugins(focus::plugin)
        .add_systems(
            Startup,
            (register_required_components, register_component_hooks),
        )
        .add_systems(
            Update,
            interact.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        );
}

/// Indicates whether an entity can be interacted with
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
pub struct Interactable;

impl Interactable {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        info!("On add interactable {:?}", ctx.entity);
        // This is used for finding focus target; see focus
        world.commands().entity(ctx.entity).insert((
            CollisionLayers::new(GameLayer::Interactable, LayerMask::ALL),
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

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Fuse(pub bool);

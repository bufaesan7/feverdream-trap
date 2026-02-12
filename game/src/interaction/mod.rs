mod focus;
mod interactions;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshTag,
    pbr::ExtendedMaterial,
};

#[cfg(feature = "dev")]
use crate::interaction::interactions::DebugInteraction;
use crate::{interaction::focus::FocusTarget, prelude::*};

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
        // This is used for finding focus target; see focus
        world.commands().entity(ctx.entity).insert((
            CollisionLayers::new(GameLayer::Interactable, LayerMask::ALL),
            #[cfg(feature = "dev")]
            DebugInteraction,
        ));

        // Replace StandardMaterial with ExtendedMaterial
        let Some(standard_material) = world
            .entity(ctx.entity)
            .get::<MeshMaterial3d<StandardMaterial>>()
        else {
            return;
        };

        let Some(standard_materials) = world.get_resource::<Assets<StandardMaterial>>() else {
            return;
        };

        let Some(standard_material) = standard_materials.get(standard_material).cloned() else {
            return;
        };

        let Some(colors) = world.get_resource::<HighlightStorageBuffer>() else {
            return;
        };
        let colors = colors.0.clone();

        let Some(mut extended_materials) = world
            .get_resource_mut::<Assets<ExtendedMaterial<StandardMaterial, HighlightExtension>>>()
        else {
            return;
        };

        let extended_material = extended_materials.add(ExtendedMaterial {
            base: standard_material,
            extension: HighlightExtension { colors },
        });

        // Replace component
        world
            .commands()
            .entity(ctx.entity)
            .try_remove::<MeshMaterial3d<StandardMaterial>>();
        world
            .commands()
            .entity(ctx.entity)
            .try_insert(MeshMaterial3d(extended_material));
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

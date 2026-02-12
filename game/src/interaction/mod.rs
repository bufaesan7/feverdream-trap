mod focus;
mod interactions;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    pbr::ExtendedMaterial,
    render::storage::ShaderStorageBuffer,
};

use crate::{
    interaction::{
        focus::{FocusTarget, HighlightExtension, HighlightStorageBuffer},
        interactions::{register_component_hooks, register_required_components},
    },
    prelude::*,
};

const INTERACTION_DISTANCE: f32 = 10.0;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(focus::plugin)
        .add_systems(
            Startup,
            (register_required_components, register_component_hooks),
        )
        .add_systems(
            Update,
            interact.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
        );
}

fn recursive_replace_material(
    world: &mut DeferredWorld,
    entity: Entity,
    colors: Handle<ShaderStorageBuffer>,
) {
    info!("Recursive to {:?}", entity);

    // Replace StandardMaterial with ExtendedMaterial
    let Ok(entityref) = world.get_entity(entity) else {
        error!("0");
        return;
    };

    if let Some(standard_material) = entityref.get::<MeshMaterial3d<StandardMaterial>>().cloned() {
        let Some(standard_materials) = world.get_resource::<Assets<StandardMaterial>>() else {
            error!("1");
            return;
        };

        let Some(standard_material) = standard_materials.get(standard_material).cloned() else {
            error!("2");
            return;
        };

        let Some(mut extended_materials) = world
            .get_resource_mut::<Assets<ExtendedMaterial<StandardMaterial, HighlightExtension>>>()
        else {
            error!("3");
            return;
        };

        let extended_material = extended_materials.add(ExtendedMaterial {
            base: standard_material,
            extension: HighlightExtension {
                colors: colors.clone(),
            },
        });

        // Replace component
        world
            .commands()
            .entity(entity)
            .try_remove::<MeshMaterial3d<StandardMaterial>>();
        world
            .commands()
            .entity(entity)
            .try_insert(MeshMaterial3d(extended_material));
    }

    let Ok(entityref) = world.get_entity(entity) else {
        error!("4");
        return;
    };

    let Some(children) = entityref.get::<Children>() else {
        error!("5");
        return;
    };
    let children: Vec<Entity> = children.iter().collect();

    info!("Entity {:?} has children {:?}", entity, children);

    for child in children {
        recursive_replace_material(world, child, colors.clone());
    }
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

        let Some(colors) = world.get_resource::<HighlightStorageBuffer>() else {
            return;
        };
        let colors = colors.0.clone();

        // This needs to descend children
        recursive_replace_material(&mut world, ctx.entity, colors);
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

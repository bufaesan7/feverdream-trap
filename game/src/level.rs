use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

use crate::prelude::*;

// TODO: made redundant by asset loading
const LEVEL_WIDTH: i32 = 20;
const LEVEL_HEIGHT: i32 = 20;
const LEVEL_SIZE: f32 = 20.0;
const LEVEL_GROUND_Y: f32 = -LEVEL_SIZE / 2.0;

/// Marker component for the level entity
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Level;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(DespawnOnExit<Screen> = DespawnOnExit(Screen::Gameplay))]
/// Marker component for each [`Entity`] that is part of the level scene
pub struct LevelComponent;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[component(on_add)]
/// Abstract [`Mesh3d`] and [`MeshMaterial3d`] insertion to avoid inserting them in the
/// [`DynamicScene`] storage.
pub enum LevelComponent3d {
    Plane { size: Vec2 },
    Cube { length: f32, color: Color },
}

impl LevelComponent3d {
    fn on_add<'a>(mut world: DeferredWorld<'a>, hook: HookContext) {
        if !world.contains_resource::<Assets<Mesh>>()
            || !world.contains_resource::<Assets<StandardMaterial>>()
        {
            // Skip this hook when we're constructing a [`DynamicScene`]
            return;
        }

        let mesh_type = world.get::<LevelComponent3d>(hook.entity).unwrap().clone();

        let mut meshes: Mut<Assets<Mesh>> = world.resource_mut();
        let mesh = match mesh_type {
            LevelComponent3d::Plane { size } => meshes.add(Plane3d::new(Vec3::Y, size)),
            LevelComponent3d::Cube { length, .. } => meshes.add(Cuboid::from_length(length)),
        };

        let mut materials: Mut<Assets<StandardMaterial>> = world.resource_mut();
        let material = match mesh_type {
            LevelComponent3d::Plane { .. } => {
                materials.add(StandardMaterial::from_color(Color::WHITE))
            }
            LevelComponent3d::Cube { color, .. } => {
                materials.add(StandardMaterial::from_color(color))
            }
        };

        let mut commands = world.commands();
        commands
            .entity(hook.entity)
            .insert((Mesh3d(mesh), MeshMaterial3d(material)));
    }
}

/// Spawns a very simple level; no input at this moment
pub fn spawn_level(mut commands: Commands) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Level,
            LevelComponent,
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    // Ground
    for x in 0..LEVEL_WIDTH {
        for z in 0..LEVEL_HEIGHT {
            // TODO: If we have a single elevation, we can just have a giant plane
            let ground = commands
                .spawn((
                    Name::new("Ground"),
                    position_to_transform(x, LEVEL_GROUND_Y, z),
                    Visibility::default(),
                    LevelComponent,
                    LevelComponent3d::Plane {
                        size: Vec2::splat(LEVEL_SIZE / 2.),
                    },
                ))
                .id();

            commands.entity(level).add_child(ground);
        }
    }

    // Walls
    for z in [-1, LEVEL_HEIGHT] {
        for x in 0..LEVEL_WIDTH {
            let wall = commands
                .spawn((
                    Name::new("Wall"),
                    position_to_transform(x, 0.0, z),
                    Visibility::default(),
                    LevelComponent,
                    LevelComponent3d::Cube {
                        length: LEVEL_SIZE,
                        color: Color::BLACK,
                    },
                ))
                .id();

            commands.entity(level).add_child(wall);
        }
    }
    for x in [-1, LEVEL_WIDTH] {
        for z in 0..LEVEL_HEIGHT {
            let wall = commands
                .spawn((
                    Name::new("Wall"),
                    position_to_transform(x, 0.0, z),
                    Visibility::default(),
                    LevelComponent,
                    LevelComponent3d::Cube {
                        length: LEVEL_SIZE,
                        color: Color::BLACK,
                    },
                ))
                .id();

            commands.entity(level).add_child(wall);
        }
    }

    commands.spawn((
        Name::new("Plane"),
        Transform::default(),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        LevelComponent,
        LevelComponent3d::Plane {
            size: Vec2::splat(10.),
        },
    ));

    commands.spawn((
        Name::new("Cube"),
        Transform::from_xyz(0., 0., -20.),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        LevelComponent,
        LevelComponent3d::Cube {
            length: 3.,
            color: bevy::color::palettes::css::BLUE.into(),
        },
    ));
}

fn position_to_transform(x: i32, y: f32, z: i32) -> Transform {
    // TODO: Offset by some amount for now, because I dont want to move camera :D
    Transform::from_xyz((x - 1) as f32 * LEVEL_SIZE, y, (z - 1) as f32 * LEVEL_SIZE)
}

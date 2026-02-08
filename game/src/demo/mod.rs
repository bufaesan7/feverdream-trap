//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use crate::prelude::*;

mod animation;
pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
}

pub fn demo_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Plane"),
        Transform::default(),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.)))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
    ));

    commands.spawn((
        Name::new("Cube"),
        Transform::from_xyz(0., 0., -20.),
        Visibility::Visible,
        DespawnOnExit(Screen::Gameplay),
        Mesh3d(meshes.add(Cuboid::new(3., 3., 3.))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(
            bevy::color::palettes::css::BLUE,
        ))),
    ));
}

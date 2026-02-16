//! The game's main screen states and transitions between them.

mod game_over;
mod gameplay;
mod loading;
mod splash;
mod title;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        game_over::plugin,
    ));

    app.add_systems(Startup, spawn_ui_camera);
    app.add_systems(OnExit(Screen::Gameplay), spawn_ui_camera);
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
    GameOver,
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera 2d"),
        Camera2d,
        DespawnOnEnter(Screen::Gameplay),
    ));
}

//! The screen state for the game over screen

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Toggle pause on keypress.
    app.add_systems(OnEnter(Screen::GameOver), spawn_game_over_screen)
        .add_systems(
            Update,
            exit_game_over
                .run_if(in_state(Screen::GameOver).and(input_just_pressed(KeyCode::Escape))),
        );
}

fn spawn_game_over_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Game Over Screen"),
        DespawnOnExit(Screen::GameOver),
        children![widget::label("YOU ESCAPED!")],
    ));
}

fn exit_game_over(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Splash);
}

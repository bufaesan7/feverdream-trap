use bevy::window::{CursorGrabMode, CursorOptions};

use crate::prelude::*;

pub fn cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn cursor_ungrab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;
}

pub fn toggle_cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = !cursor_options.visible;

    cursor_options.grab_mode = match cursor_options.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        _ => CursorGrabMode::None,
    }
}

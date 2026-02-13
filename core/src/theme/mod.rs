//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette};
}

use crate::prelude::*;

pub(super) fn plugin<S: States>(app: &mut App, state: Option<S>) {
    interaction::plugin(app, state);
}

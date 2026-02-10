pub use crate::asset_plugin;
pub use crate::asset_tracking::{LoadResource as _, ResourceHandles};
pub use crate::audio::{music, sound_effect};
pub use crate::theme::widget;

/// Bevy & ecosystem exports
pub use avian3d::prelude::*;
pub use bevy::input::common_conditions::input_just_pressed;
pub use bevy::platform::collections::HashMap;
pub use bevy::prelude::*;
#[cfg(feature = "dev")]
pub use bevy_egui::PrimaryEguiContext;
#[cfg(feature = "dev")]
pub use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};

/// Common std imports
pub use std::f32::consts::{PI, TAU};
pub use std::time::Duration;

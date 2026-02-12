pub use crate::asset_loader::RonAsset as _;
pub use crate::asset_plugin;
pub use crate::asset_tracking::{LoadResource as _, ResourceHandles};
pub use crate::chunk::*;
pub use crate::chunk_assets::*;
pub use crate::level::*;
pub use crate::physics::*;
pub use crate::theme::widget;
pub use crate::utils::audio::{music, sound_effect};
pub use crate::utils::*;

/// Bevy & ecosystem exports
pub use avian3d::prelude::*;
pub use bevy::input::common_conditions::input_just_pressed;
pub use bevy::platform::collections::HashMap;
pub use bevy::prelude::*;
#[cfg(feature = "dev_native")]
pub use bevy_egui::PrimaryEguiContext;
#[cfg(feature = "dev_native")]
pub use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};

/// Common std imports
pub use std::f32::consts::{PI, TAU};
pub use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub use ron::to_string;

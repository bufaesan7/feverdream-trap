mod asset_loader;
mod asset_tracking;
pub mod chunk;
pub mod chunk_assets;
pub mod interactions;
pub mod level;
pub mod physics;
pub mod prelude;
mod theme;
pub(crate) mod utils;

use crate::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy_skein::SkeinPlugin;

pub fn utility_plugin(app: &mut App) {
    app.add_plugins(SkeinPlugin::default());

    app.add_plugins((
        asset_tracking::plugin,
        chunk_assets::plugin,
        chunk::plugin,
        utils::audio::plugin,
        theme::plugin,
    ));
}

pub fn asset_plugin() -> AssetPlugin {
    #[allow(unused_mut)]
    let mut asset_plugin = AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..Default::default()
    };

    #[cfg(all(feature = "dev", not(target_arch = "wasm32")))]
    {
        let package = env!("CARGO_PKG_NAME").split("_").last().unwrap();
        let asset_path = env!("CARGO_MANIFEST_DIR").replacen(package, "assets", 1);

        asset_plugin.file_path = asset_path;
    }

    asset_plugin
}

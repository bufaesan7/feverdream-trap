pub mod prelude;

use crate::prelude::*;
use bevy::asset::AssetMetaCheck;

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

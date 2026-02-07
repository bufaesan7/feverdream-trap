use bevy::{asset::AssetMetaCheck, prelude::*};

pub mod prelude;

pub fn asset_plugin() -> AssetPlugin {
    #[cfg(all(feature = "dev", not(target_arch = "wasm32")))]
    {
        let package = env!("CARGO_PKG_NAME").split("_").last().unwrap();
        let asset_path = env!("CARGO_MANIFEST_DIR").replacen(package, "assets", 1);

        AssetPlugin {
            file_path: asset_path,
            meta_check: AssetMetaCheck::Never,
            ..default()
        }
    }
    #[cfg(any(not(feature = "dev"), target_arch = "wasm32"))]
    {
        AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }
    }
}

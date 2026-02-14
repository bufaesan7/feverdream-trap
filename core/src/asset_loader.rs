use std::marker::PhantomData;

use bevy::asset::{AssetLoader, AsyncReadExt};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::prelude::*;

pub trait RonAsset: TypePath + DeserializeOwned + Send + Sync + 'static {
    type Asset: Asset;
    const EXTENSION: &str;

    fn load_dependencies(
        self,
        context: &mut bevy::asset::LoadContext<'_>,
    ) -> impl Future<Output = Self::Asset> + Send + Sync;
}

#[derive(TypePath)]
pub struct RonAssetLoader<T> {
    _phantom: PhantomData<T>,
}

impl<T> RonAssetLoader<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: RonAsset> AssetLoader for RonAssetLoader<T> {
    type Asset = T::Asset;
    type Settings = ();
    type Error = RonAssetLoadError;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        async {
            let mut bytes = String::new();
            reader.read_to_string(&mut bytes).await?;
            let t: T = ron::from_str(&bytes)?;
            Ok(T::load_dependencies(t, load_context).await)
        }
    }
}

#[derive(Error, Debug)]
pub enum RonAssetLoadError {
    #[error("Asset reader failed: {0}")]
    AssetReaderFailed(#[from] bevy::tasks::futures_lite::io::Error),
    #[error("Asset deserialization failed: {0}")]
    DeserializationError(#[from] ron::error::SpannedError),
}

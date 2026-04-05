//! Asset loader for Sprite Fusion map files.

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use thiserror::Error;

use crate::types::SpriteFusionMap;

/// Asset loader for SpriteFusion JSON map files.
#[derive(Default, Reflect)]
pub struct SpriteFusionMapLoader;

/// Errors that can occur when loading a SpriteFusion map.
#[derive(Debug, Error)]
pub enum SpriteFusionMapLoaderError {
    #[error("Failed to read map file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse map JSON: {0}")]
    Json(#[from] serde_json::Error),
}

impl AssetLoader for SpriteFusionMapLoader {
    type Asset = SpriteFusionMap;
    type Settings = ();
    type Error = SpriteFusionMapLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let map: SpriteFusionMap = serde_json::from_slice(&bytes)?;
        Ok(map)
    }

    fn extensions(&self) -> &[&str] {
        &["sf.json"]
    }
}

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::{BoxedFuture, ConditionalSendFuture},
};
use serde::Deserialize;
use thiserror::Error;

pub struct BinaryDataPlugin;

impl Plugin for BinaryDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BinaryData>()
            .init_asset_loader::<BinaryDataLoader>();
    }
}

#[derive(Asset, Debug, TypePath, Deserialize)]
pub struct BinaryData(pub Vec<u8>);

#[derive(Default)]
pub struct BinaryDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BinaryDataLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    // #[error("Could not parse the asset {0}")]
    // XmlParseError(#[from] std::error::Error),
}

impl AssetLoader for BinaryDataLoader {
    type Asset = BinaryData;
    type Settings = ();
    type Error = BinaryDataLoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(BinaryData(bytes))
        }
    }

    fn extensions(&self) -> &[&str] {
        &["bin"]
    }
}

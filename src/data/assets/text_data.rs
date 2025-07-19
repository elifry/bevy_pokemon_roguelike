use std::string::FromUtf8Error;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::{BoxedFuture, ConditionalSendFuture},
};
use serde::Deserialize;
use thiserror::Error;

pub struct TextDataPlugin;

impl Plugin for TextDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<TextAsset>()
            .init_asset_loader::<TextDataLoader>();
    }
}

#[derive(Asset, Debug, TypePath, Deserialize)]
pub struct TextAsset(pub String);

#[derive(Default)]
pub struct TextDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TextDataLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse the asset {0}")]
    StringParseError(#[from] FromUtf8Error),
}

impl AssetLoader for TextDataLoader {
    type Asset = TextAsset;
    type Settings = ();
    type Error = TextDataLoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let text = String::from_utf8(bytes)?;

            Ok(TextAsset(text))
        }
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

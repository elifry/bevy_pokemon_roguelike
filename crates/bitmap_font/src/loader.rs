use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use bincode::error::DecodeError;
use thiserror::Error;

use crate::BitmapFont;

pub struct BitmapFontPlugin;

impl Plugin for BitmapFontPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BitmapFont>()
            .init_asset_loader::<FontSheetDataLoader>();
    }
}

#[derive(Default)]
pub struct FontSheetDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BinaryDataLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse the asset {0}")]
    DecodeError(#[from] DecodeError),
}

impl AssetLoader for FontSheetDataLoader {
    type Asset = BitmapFont;
    type Settings = ();
    type Error = BinaryDataLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let font_sheet_data = BitmapFont::load(&bytes)?;

            Ok(font_sheet_data)
        })
    }

    fn extensions(&self) -> &[&str] {
        &[".bfn"]
    }
}

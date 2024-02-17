use std::sync::Arc;

use crate::bfn;
use bevy::{
    asset::{io::Reader, meta::AssetMeta, AssetLoader, AsyncReadExt, LoadContext, LoadedAsset},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::BoxedFuture,
};
use bincode::error::DecodeError;
use thiserror::Error;

#[derive(Debug)]
pub struct BitmapFontData {
    pub texture: Handle<Image>,
    pub font: bfn::Font,
}

/// A bitmap font asset that can be loaded from .bfn files
#[derive(TypePath, Asset)]
pub struct BitmapFont {
    pub data: Arc<BitmapFontData>,
}

#[derive(Default)]
pub struct BitmapFontLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BinaryDataLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse the asset {0}")]
    DecodeError(#[from] DecodeError),
}

impl AssetLoader for BitmapFontLoader {
    type Asset = BitmapFont;
    type Settings = ();
    type Error = BinaryDataLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let font = bfn::Font::load(&bytes)?;

            let texture_buffer = image::load_from_memory(&font.texture)
                .expect("Failed to decompress font bitmap texture")
                .to_rgba8();

            let texture = Image::new(
                Extent3d {
                    width: font.size.0 as u32,
                    height: font.size.1 as u32,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                texture_buffer.into_raw(),
                TextureFormat::Rgba8Unorm,
            );

            // let labeled = load_context.begin_labeled_asset();
            // let loaded_asset = labeled.finish(texture, None);
            // let handle = load_context.add_loaded_labeled_asset("font_texture", loaded_asset);

            let texture_handle = load_context
                .add_loaded_labeled_asset("texture".to_string(), LoadedAsset::from(texture));

            let bitmap_font = BitmapFont {
                data: Arc::new(BitmapFontData {
                    font,
                    texture: texture_handle,
                }),
            };

            Ok(bitmap_font)
        })
    }

    fn extensions(&self) -> &[&str] {
        &[".bfn"]
    }
}

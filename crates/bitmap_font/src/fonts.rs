use std::{collections::HashMap, sync::Arc};

use crate::bfn;
use bevy::{
    asset::{io::Reader, meta::AssetMeta, AssetLoader, AsyncReadExt, LoadContext, LoadedAsset},
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils::BoxedFuture,
};
use bevy_egui::egui;
use bincode::error::DecodeError;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct BitmapFontData {
    pub texture: Handle<Image>,
    pub font: bfn::Font,
    pub glyph_uvs: HashMap<u32, egui::Rect>,
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
pub enum BitmapFontLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse the asset {0}")]
    DecodeError(#[from] DecodeError),
}

impl AssetLoader for BitmapFontLoader {
    type Asset = BitmapFont;
    type Settings = ();
    type Error = BitmapFontLoaderError;

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

            // Build glyph uvs
            let mut glyph_uvs = HashMap::default();
            for glyph in font.glyphs.values() {
                let bounds = &glyph.bounds;
                let width = font.size.0 as f32;
                let height = font.size.1 as f32;

                // Skip white space
                if char::from_u32(glyph.code_point).unwrap().is_whitespace() {
                    continue;
                }

                let glyph_uv = egui::Rect::from_min_size(
                    egui::Pos2::new(
                        (glyph.bounds.x as f32 / width),
                        glyph.bounds.y as f32 / height,
                    ),
                    egui::Vec2::new(
                        glyph.bounds.width as f32 / width,
                        glyph.bounds.height as f32 / height,
                    ),
                );

                glyph_uvs.insert(glyph.code_point, glyph_uv);
            }

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
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
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
                    glyph_uvs,
                }),
            };

            Ok(bitmap_font)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["bfn"]
    }
}

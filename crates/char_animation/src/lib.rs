use std::collections::HashMap;

use crate::{anim_key::AnimKey, orientation::Orientation};
use bevy::{
    app::{App, Plugin},
    asset::{
        io::Reader, Asset, AssetApp, AssetLoader, AsyncReadExt, Handle, LoadContext, LoadedAsset,
    },
    math::Vec2,
    reflect::TypePath,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::Image,
    },
    sprite::TextureAtlasLayout,
    utils::BoxedFuture,
};
use bincode::error::DecodeError;
use file::{CharAnimationFile, CharAnimationOffsets};
use strum::IntoEnumIterator;
use thiserror::Error;

pub mod anim_key;
pub mod file;
pub mod orientation;

pub struct CharAnimationPlugin;

impl Plugin for CharAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharAnimation>()
            .init_asset_loader::<CharAnimationLoader>();
    }
}

#[derive(Debug, Clone)]
pub struct CharAnimationData {
    // Texture / Atlas
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,

    // Frames info
    pub index: usize,
    pub frame_width: u32,
    pub frame_height: u32,
    pub durations: Vec<u32>,
    pub rush_frame: Option<usize>,
    pub hit_frame: Option<usize>,
    pub return_frame: Option<usize>,

    // Offsets
    pub shadow_offsets: HashMap<Orientation, Vec<Vec2>>,
    pub offsets: HashMap<Orientation, Vec<CharAnimationOffsets>>,
}

/// A bitmap font asset that can be loaded from .bfn files
#[derive(TypePath, Asset, Debug)]
pub struct CharAnimation {
    pub anim: HashMap<AnimKey, CharAnimationData>,
}

#[derive(Default)]
pub struct CharAnimationLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CharAnimationLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse the asset {0}")]
    DecodeError(#[from] DecodeError),
}

impl AssetLoader for CharAnimationLoader {
    type Asset = CharAnimation;
    type Settings = ();
    type Error = CharAnimationLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let char_animation_file = CharAnimationFile::load(&bytes)?;

            let char_animations = char_animation_file
                .anim
                .iter()
                .map(|(anim_key, char_animation_entry)| {
                    let texture_buffer = image::load_from_memory(&char_animation_entry.texture)
                        .expect("Failed to decompress char animation texture")
                        .to_rgba8();

                    let texture = Image::new(
                        Extent3d {
                            width: texture_buffer.width(),
                            height: texture_buffer.height(),
                            depth_or_array_layers: 1,
                        },
                        TextureDimension::D2,
                        texture_buffer.into_raw(),
                        TextureFormat::Rgba8UnormSrgb,
                        // Unload the texture inside de VRAM
                        RenderAssetUsages::RENDER_WORLD,
                    );

                    let texture_label = format!("{}_texture.png", anim_key);
                    let texture_handle = load_context
                        .add_loaded_labeled_asset(texture_label, LoadedAsset::from(texture));

                    let tile_size = Vec2::new(
                        char_animation_entry.frame_width as f32,
                        char_animation_entry.frame_height as f32,
                    );

                    let rows = match char_animation_entry.is_single_orientation {
                        false => Orientation::iter().len(),
                        true => 1,
                    };

                    let atlas_layout = TextureAtlasLayout::from_grid(
                        tile_size,
                        char_animation_entry.durations.len(),
                        rows,
                        None,
                        None,
                    );

                    let atlas_layout_label = format!("{}_atlas_layout_label", anim_key);
                    let atlas_layout_handle = load_context.add_loaded_labeled_asset(
                        atlas_layout_label,
                        LoadedAsset::from(atlas_layout),
                    );

                    // TODO: find a better way than cloning the char_animation_entry
                    let data = CharAnimationData {
                        texture: texture_handle,
                        atlas_layout: atlas_layout_handle,
                        index: char_animation_entry.index,
                        frame_width: char_animation_entry.frame_width,
                        frame_height: char_animation_entry.frame_height,
                        durations: char_animation_entry.durations.to_owned(),
                        rush_frame: char_animation_entry.rush_frame,
                        hit_frame: char_animation_entry.hit_frame,
                        return_frame: char_animation_entry.return_frame,
                        shadow_offsets: char_animation_entry.shadow_offsets.to_owned(),
                        offsets: char_animation_entry.offsets.to_owned(),
                    };
                    (*anim_key, data)
                })
                .collect::<HashMap<_, _>>();

            Ok(CharAnimation {
                anim: char_animations,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["chara"]
    }
}

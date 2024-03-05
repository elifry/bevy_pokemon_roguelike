use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use thiserror::Error;

pub mod data;

pub use data::*;

pub struct PokemonDataPlugin;

impl Plugin for PokemonDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<PokemonData>()
            .init_asset_loader::<PokemonDataLoader>();
    }
}

#[derive(Default)]
pub struct PokemonDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PokemonDataLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse the asset {0}")]
    DecodeError(#[from] ron::Error),
}

impl AssetLoader for PokemonDataLoader {
    type Asset = PokemonData;
    type Settings = ();
    type Error = PokemonDataLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let pokemon_data = PokemonData::load(&bytes)?;
            Ok(pokemon_data)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pd.ron"]
    }
}

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

use crate::graphics::anim_data::AnimData;
use crate::pokemons::Pokemons;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    // #[asset(key = "tiles.forest_path")]
    #[asset(texture_atlas(
        tile_size_x = 24.,
        tile_size_y = 24.,
        columns = 21,
        rows = 24,
        padding_x = 1.,
        padding_y = 1.,
        offset_x = 1.,
        offset_y = 1.
    ))]
    #[asset(path = "images/tiles/forest_path_tiles.png")]
    pub forest_path: Handle<TextureAtlas>,
}

#[derive(Resource, Debug, Default)]
pub struct PokemonAnimationAssets(pub HashMap<Pokemons, PokemonAnimation>);

#[derive(Default, Resource)]
pub struct PokemonAssetsFolder(pub HashMap<String, Handle<LoadedFolder>>);

#[derive(Debug)]
pub struct PokemonAnimation {
    pub idle: Handle<TextureAtlas>,
    pub walk: Handle<TextureAtlas>,
    pub attack: Handle<TextureAtlas>,
    pub anim_data: Handle<AnimData>,
}

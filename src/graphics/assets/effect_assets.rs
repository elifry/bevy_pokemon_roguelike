use std::str::FromStr;

use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

use crate::effects::Effect;
use crate::graphics::anim_data::{AnimData, AnimKey};
use crate::pokemons::Pokemons;
use crate::GameState;

pub struct EffectAssetsPlugin;

impl Plugin for EffectAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EffectAssetsFolder>()
            .init_resource::<EffectAssets>()
            .add_systems(OnEnter(GameState::AssetsLoaded), process_effect_assets);
    }
}

#[derive(Default, Resource)]
pub struct EffectAssetsFolder(pub HashMap<String, Handle<LoadedFolder>>);

#[derive(Resource, Debug, Default)]
pub struct EffectAssets(pub HashMap<Effect, EffectAssets>);

#[derive(Debug, Clone)]
pub struct EffectAsset {
    pub textures: HashMap<&'static str, Handle<TextureAtlas>>,
}

fn process_effect_assets(
    effect_assets_folder: ResMut<EffectAssetsFolder>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut effect_assets: ResMut<EffectAssets>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
}

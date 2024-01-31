use std::path::Path;
use std::str::FromStr;

use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_asset_loader::prelude::*;
use itertools::Itertools;

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
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    for (effect, handle_folder) in effect_assets_folder.0.iter() {
        let Some::<&LoadedFolder>(folder) = loaded_folder_assets.get(handle_folder) else {
            error!("Could'nt load the folder for effect {}", effect);
            continue;
        };

        let effect = Effect::from_str(&effect.to_string()).unwrap();

        let parent_path_str = format!("effects/{}", effect.clone());
        let parent = Path::new(&parent_path_str);

        // Set every effect image in the map by its sub type
        let mut effects_by_sub_type: HashMap<&str, Vec<Handle<Image>>> = HashMap::new();
        for handle in folder.handles.iter() {
            if let Some(path_buf) = handle.path() {
                let path = path_buf.path();
                if path.ancestors().any(|ancestor| ancestor == parent)
                    && path
                        .parent()
                        .map_or(false, |p| p.file_name().unwrap_or_default() != "pieces")
                {
                    if let Some(sub_type) = path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                    {
                        let entry = effects_by_sub_type.entry(sub_type).or_insert_with(Vec::new);

                        entry.push(handle.to_owned().typed::<Image>());
                    }
                }
            }
        }

        // Sort images by its file name
        for images in effects_by_sub_type.values_mut() {
            images.sort_by_key(|image| {
                image
                    .path()
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            });
        }

        let mut effect_texture_atlases: HashMap<&str, Handle<TextureAtlas>> = HashMap::new();

        for (sub_type, images) in effects_by_sub_type.into_iter() {
            let mut builder = TextureAtlasBuilder::default();
            for handle in images {
                let id = handle.id();

                let Some(texture) = textures.get(id) else {
                    warn!("Texture not loaded: {:?}", handle.path().unwrap());
                    continue;
                };

                builder.add_texture(id, texture)
            }

            let atlas = builder.finish(&mut textures).unwrap();
            let atlas_handle = texture_atlasses.add(atlas);
            effect_texture_atlases.insert(sub_type, atlas_handle);
        }

        // Clean up unused resources
        commands.remove_resource::<EffectAssetsFolder>();
    }
}

use std::path::Path;
use std::str::FromStr;

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::effects::EffectID;

use crate::utils::get_path_from_handle;
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
pub struct EffectAssets(pub HashMap<EffectID, EffectAsset>);

#[derive(Debug, Clone)]
pub struct EffectAsset {
    pub textures: HashMap<String, Handle<TextureAtlas>>,
}

fn process_effect_assets(
    effect_assets_folder: Res<EffectAssetsFolder>,
    mut effect_assets: ResMut<EffectAssets>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    for (effect, handle_folder) in effect_assets_folder.0.iter() {
        let folder = match loaded_folder_assets.get(handle_folder) {
            Some(folder) => folder,
            None => {
                error!("Couldn't load the folder for effect {}", effect);
                continue;
            }
        };

        let effect = match EffectID::from_str(&effect.to_string()) {
            Ok(effect) => effect,
            Err(_) => {
                error!("Invalid effect: {}", effect);
                continue;
            }
        };

        let parent_path_str = format!("effects/{}", effect);
        let parent = Path::new(&parent_path_str);

        let mut effects_by_sub_type: HashMap<String, Vec<Handle<Image>>> = HashMap::new();
        for handle in folder.handles.iter() {
            let Some(path) = get_path_from_handle(handle) else {
                continue;
            };
            if path.ancestors().all(|ancestor| ancestor != parent) {
                continue;
            }

            let Some(sub_type) = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
            else {
                continue;
            };

            if sub_type != "pieces" {
                effects_by_sub_type
                    .entry(sub_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(handle.to_owned().typed::<Image>());
            }
        }

        // Sorting and atlas building remains largely the same, with error handling improved
        for images in effects_by_sub_type.values_mut() {
            images.sort_by_key(|image| {
                get_path_from_handle(&image.clone().untyped())
                    .and_then(|path| path.file_name()?.to_str())
                    .unwrap_or_default()
                    .to_owned()
            });
        }

        let mut effect_texture_atlases: HashMap<String, Handle<TextureAtlas>> = HashMap::new();

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

        effect_assets.0.insert(
            effect,
            EffectAsset {
                textures: effect_texture_atlases,
            },
        );
    }
    // // Clean up unused resources
    commands.remove_resource::<EffectAssetsFolder>();
}

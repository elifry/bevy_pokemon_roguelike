use std::path::Path;
use std::str::FromStr;

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::effects::Effect;
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
pub struct EffectAssets(pub HashMap<Effect, EffectAsset>);

#[derive(Debug, Clone)]
pub struct EffectAsset {
    pub textures: HashMap<String, EffectTextureInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct EffectTextureInfo {
    pub texture_atlas: Handle<TextureAtlas>,
    /// Contains the correct order for displaying animation from the texture atlas
    /// TODO: Handle empty frame (example, if there 004 then 006, it means 005 should be an empty frame)
    pub frame_order: Vec<usize>,
}

fn process_effect_assets(
    effect_assets_folder: Res<EffectAssetsFolder>,
    mut effect_assets: ResMut<EffectAssets>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

        let effect = match Effect::from_str(&effect.to_string()) {
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

        let mut effect_texture_atlases: HashMap<String, EffectTextureInfo> = HashMap::new();

        for (sub_type, images) in effects_by_sub_type.into_iter() {
            let mut builder = TextureAtlasBuilder::default();

            for handle in &images {
                let id: AssetId<Image> = handle.id();
                println!("{:?}", handle);

                let Some(texture) = textures.get(id) else {
                    warn!("Texture not loaded: {:?}", handle.path().unwrap());
                    continue;
                };

                builder.add_texture(id, texture)
            }

            let atlas = builder.finish(&mut textures).unwrap();

            let mut frame_order: Vec<usize> = Vec::new();
            for handle in images {
                let texture_index = atlas.get_texture_index(handle).unwrap();
                frame_order.push(texture_index);
            }

            let atlas_handle = texture_atlases.add(atlas);
            effect_texture_atlases.insert(
                sub_type,
                EffectTextureInfo {
                    texture_atlas: atlas_handle,
                    frame_order,
                },
            );
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

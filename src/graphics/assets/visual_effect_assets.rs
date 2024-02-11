use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::utils::get_path_from_handle;
use crate::GameState;

use super::AssetsLoading;

const VISUAL_EFFECTS_PATH: &str = "visual_effects";

pub struct VisualEffectAssetsPlugin;

impl Plugin for VisualEffectAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisualEffectAssetsFolder>()
            .init_resource::<VisualEffectAssets>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_effect_assets);
    }
}

#[derive(Debug, Default, Clone)]
pub enum VisualEffectDirectionType {
    #[default]
    None,
    Dir5,
    Dir8,
}

#[derive(Default, Resource)]
struct VisualEffectAssetsFolder(Handle<LoadedFolder>);

#[derive(Resource, Debug, Default)]
pub struct VisualEffectAssets(pub HashMap<String, VisualEffectAsset>);

#[derive(Debug, Clone)]
pub struct VisualEffectAsset {
    pub texture_atlas: Handle<TextureAtlas>,
    pub direction: VisualEffectDirectionType,
}

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut visual_effect_assets_folder: ResMut<VisualEffectAssetsFolder>,
) {
    info!("visual effect assets loading...");

    // Visual Effects
    let visual_effect_folder = asset_server.load_folder(VISUAL_EFFECTS_PATH);
    loading.0.push(visual_effect_folder.clone().untyped());
    visual_effect_assets_folder.0 = visual_effect_folder;
}

fn process_effect_assets(
    visual_effect_assets_folder: Res<VisualEffectAssetsFolder>,
    mut visual_effect_assets: ResMut<VisualEffectAssets>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: Res<Assets<Image>>,
) {
    let folder: &LoadedFolder = match loaded_folder_assets.get(&visual_effect_assets_folder.0) {
        Some(folder) => folder,
        None => {
            error!("Couldn't load the visual effects folder");
            return;
        }
    };

    let mut visual_effect_images = Vec::new();
    for vf_handle in folder.handles.iter() {
        let Some(path) = get_path_from_handle(vf_handle) else {
            continue;
        };
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let direction = if file_name.ends_with("Dir5.png") {
            VisualEffectDirectionType::Dir5
        } else if file_name.ends_with("Dir8.png") {
            VisualEffectDirectionType::Dir8
        } else if file_name.ends_with("None.png") {
            VisualEffectDirectionType::None
        } else {
            continue;
        };
        visual_effect_images.push((direction, file_name, vf_handle.to_owned().typed::<Image>()))
    }

    for (direction, file_name, vf_image) in visual_effect_images {
        let Some(vf_texture) = textures.get(vf_image.id()) else {
            warn!("Texture not loaded: {:?}", vf_image.path().unwrap());
            continue;
        };

        let texture_atlas = match direction {
            VisualEffectDirectionType::None => {
                let tile_size = vf_texture.height();
                TextureAtlas::from_grid(
                    vf_image,
                    Vec2::splat(tile_size as f32),
                    (vf_texture.width() / tile_size) as usize,
                    1,
                    None,
                    None,
                )
            }
            VisualEffectDirectionType::Dir5 => {
                let tile_size = vf_texture.height() / 5;
                TextureAtlas::from_grid(
                    vf_image,
                    Vec2::splat(tile_size as f32),
                    (vf_texture.width() / tile_size) as usize,
                    5,
                    None,
                    None,
                )
            }
            VisualEffectDirectionType::Dir8 => {
                let tile_size = vf_texture.height() / 8;
                TextureAtlas::from_grid(
                    vf_image,
                    Vec2::splat(tile_size as f32),
                    (vf_texture.width() / tile_size) as usize,
                    8,
                    None,
                    None,
                )
            }
        };

        let atlas_handle = texture_atlases.add(texture_atlas);

        let visual_effect_texture_info = VisualEffectAsset {
            texture_atlas: atlas_handle,
            direction,
        };

        let Some((visual_effect_name, _)) = file_name.split_once('.') else {
            warn!(
                "Couldn't extract the key for the visual effect for {}",
                file_name
            );
            continue;
        };

        visual_effect_assets
            .0
            .insert(visual_effect_name.to_owned(), visual_effect_texture_info);
    }

    // commands.remove_resource::<VisualEffectAssetsFolder>();
}

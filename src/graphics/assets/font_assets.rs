use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use font_atlas::FontSheetData;
use serde::Deserialize;

use crate::utils::find_first_handle_by_extension;
use crate::GameState;

use super::AssetsLoading;

const FONTS_PATH: &str = "fonts";

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FontAssetsFolder(default()))
            .init_resource::<FontAssets>()
            .init_asset::<FontSheet>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_font_assets);
    }
}

/// Store glyph information
#[derive(Debug, Deserialize, Default, PartialEq, PartialOrd, Clone)]
pub struct FontGlyph {
    pub index: usize,
    pub color_less: bool,
}

/// Store all glyph information for a font
#[derive(Asset, TypePath, Debug, Deserialize, Default)]
pub struct FontSheet {
    pub glyphs: HashMap<u32, FontGlyph>,
}

/// Store the font sheet and the texture atlas for a font
#[derive(Debug, Default, Reflect, Clone)]
pub struct FontAsset {
    pub texture_atlas: Handle<TextureAtlas>,
    pub font_sheet: Handle<FontSheet>,
}

#[derive(Resource, Debug, Default)]
pub struct FontAssets(pub HashMap<String, FontAsset>);

#[derive(Default, Resource)]
struct FontAssetsFolder(HashMap<String, Handle<LoadedFolder>>);

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut font_assets_folder: ResMut<FontAssetsFolder>,
) {
    info!("font assets loading...");

    // Fonts
    let font_to_load_list = vec!["text"];
    for font_to_load in font_to_load_list {
        let font_folder = asset_server.load_folder(format!("{FONTS_PATH}/{font_to_load}"));
        loading.0.push(font_folder.clone().untyped());
        font_assets_folder
            .0
            .insert(font_to_load.to_string(), font_folder);
    }
}

fn process_font_assets(
    font_assets_folder: Res<FontAssetsFolder>,
    mut font_assets: ResMut<FontAssets>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut font_sheets: ResMut<Assets<FontSheet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    font_sheet_data_assets: Res<Assets<FontSheetData>>,
    mut commands: Commands,
) {
    for (font, handle_folder) in font_assets_folder.0.iter() {
        let Some::<&LoadedFolder>(folder) = loaded_folder_assets.get(handle_folder) else {
            error!("Could'nt load the folder for {}", font);
            continue;
        };

        let font_sheet_data_handle: Handle<FontSheetData> =
            find_first_handle_by_extension(&folder.handles, "data")
                .expect("No font sheet handle found");

        let texture_handle: Handle<Image> = find_first_handle_by_extension(&folder.handles, "png")
            .expect("No texture handle found");

        let font_sheet_data = font_sheet_data_assets
            .get(font_sheet_data_handle.id())
            .unwrap();

        let mut texture_atlas = TextureAtlas::new_empty(
            texture_handle,
            Vec2 {
                x: font_sheet_data.width as f32,
                y: font_sheet_data.height as f32,
            },
        );

        let mut glyphs = HashMap::with_capacity(font_sheet_data.characters.len());
        for (id, glyph_data) in font_sheet_data.characters.iter() {
            let index = texture_atlas.add_texture(glyph_data.rect);
            glyphs.insert(
                *id,
                FontGlyph {
                    index,
                    color_less: false,
                },
            );
        }

        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let font_sheet_handle = font_sheets.add(FontSheet { glyphs });
        font_assets.0.insert(
            font.to_owned(),
            FontAsset {
                texture_atlas: texture_atlas_handle,
                font_sheet: font_sheet_handle,
            },
        );
    }

    // Clean up unused resources
    commands.remove_resource::<FontAssetsFolder>();
}

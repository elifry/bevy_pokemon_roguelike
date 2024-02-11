use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use font_atlas::FontSheetData;

use crate::utils::find_first_handle_by_extension;
use crate::GameState;

use super::AssetsLoading;

const FONTS_PATH: &str = "fonts";

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FontAssetsFolder(default()))
            .init_resource::<FontSheetAsset>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_font_assets);
    }
}

#[derive(Debug)]
pub struct FontGlyph {
    pub index: usize,
    pub color_less: bool,
}

#[derive(Debug, Default)]
pub struct FontAsset {
    pub texture_atlas: Handle<TextureAtlas>,
    pub glyphs: HashMap<u32, FontGlyph>,
}

#[derive(Resource, Debug, Default)]
pub struct FontSheetAsset(pub HashMap<String, FontAsset>);

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
    mut font_sheet_assets: ResMut<FontSheetAsset>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    font_sheets: Res<Assets<FontSheetData>>,
    mut commands: Commands,
) {
    for (font, handle_folder) in font_assets_folder.0.iter() {
        let Some::<&LoadedFolder>(folder) = loaded_folder_assets.get(handle_folder) else {
            error!("Could'nt load the folder for {}", font);
            continue;
        };

        let font_sheet_handle: Handle<FontSheetData> =
            find_first_handle_by_extension(&folder.handles, "data")
                .expect("No font sheet handle found");

        let texture_handle: Handle<Image> = find_first_handle_by_extension(&folder.handles, "png")
            .expect("No texture handle found");

        let font_sheet = font_sheets.get(font_sheet_handle.id()).unwrap();

        let mut texture_atlas = TextureAtlas::new_empty(
            texture_handle,
            Vec2 {
                x: font_sheet.width as f32,
                y: font_sheet.height as f32,
            },
        );

        let mut glyphs = HashMap::with_capacity(font_sheet.characters.len());
        for (id, glyph_data) in font_sheet.characters.iter() {
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

        font_sheet_assets.0.insert(
            font.to_owned(),
            FontAsset {
                texture_atlas: texture_atlas_handle,
                glyphs,
            },
        );
    }

    // Clean up unused resources
    commands.remove_resource::<FontAssetsFolder>();
}

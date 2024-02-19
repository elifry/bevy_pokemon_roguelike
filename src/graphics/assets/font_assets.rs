use crate::GameState;
use bevy::prelude::*;
use bitmap_font::fonts::BitmapFont;

use super::AssetsLoading;

const FONTS_PATH: &str = "fonts";

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
            // .init_asset::<FontSheet>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder);
        //.add_systems(OnEnter(GameState::AssetsLoaded), process_font_assets);
    }
}

/// Store the font sheet and the texture atlas for a font
#[derive(Resource, Debug, Default, Clone)]
pub struct FontAssets {
    pub text: Handle<BitmapFont>,    // Text
    pub dungeon: Handle<BitmapFont>, // Banner
    pub damage: Handle<BitmapFont>,  // Yellow
    pub heal: Handle<BitmapFont>,    // Green
    pub exp: Handle<BitmapFont>,     // Blue
}

// /// Store glyph information
// #[derive(Debug, Deserialize, Default, PartialEq, PartialOrd, Clone)]
// pub struct FontGlyph {
//     pub index: usize,
//     pub color_less: bool,
// }

// /// Store all glyph information for a font
// #[derive(Asset, TypePath, Debug, Deserialize, Default, Clone)]
// pub struct FontSheet {
//     pub glyphs: HashMap<u32, FontGlyph>,
// }

// /// Store the font sheet and the texture atlas for a font
// #[derive(Debug, Default, Reflect, Clone)]
// #[reflect(Default)]
// pub struct FontAsset {
//     pub texture_atlas: Handle<TextureAtlas>,
//     pub font_sheet: Handle<FontSheet>,
// }

// #[derive(Resource, Debug, Default, Clone)]
// pub struct FontAssets(pub HashMap<String, FontAsset>);

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut font_assets: ResMut<FontAssets>,
) {
    info!("font assets loading...");

    // Fonts
    font_assets.text = asset_server.load(format!("{FONTS_PATH}/text.bfn"));
    loading.0.push(font_assets.text.clone().untyped());

    font_assets.dungeon = asset_server.load(format!("{FONTS_PATH}/banner.bfn"));
    loading.0.push(font_assets.dungeon.clone().untyped());

    font_assets.damage = asset_server.load(format!("{FONTS_PATH}/yellow.bfn"));
    loading.0.push(font_assets.damage.clone().untyped());

    font_assets.heal = asset_server.load(format!("{FONTS_PATH}/green.bfn"));
    loading.0.push(font_assets.heal.clone().untyped());

    font_assets.exp = asset_server.load(format!("{FONTS_PATH}/blue.bfn"));
    loading.0.push(font_assets.exp.clone().untyped());
}

// fn process_font_assets(
//     font_assets_folder: Res<FontAssetsFolder>,
//     mut font_assets: ResMut<FontAssets>,
//     loaded_folder_assets: Res<Assets<LoadedFolder>>,
//     mut font_sheets: ResMut<Assets<FontSheet>>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     font_sheet_data_assets: Res<Assets<FontSheetData>>,
//     mut commands: Commands,
// ) {
//     for (font, handle_folder) in font_assets_folder.0.iter() {
//         let Some::<&LoadedFolder>(folder) = loaded_folder_assets.get(handle_folder) else {
//             error!("Could'nt load the folder for {}", font);
//             continue;
//         };

//         let font_sheet_data_handle: Handle<FontSheetData> =
//             find_first_handle_by_extension(&folder.handles, "data")
//                 .expect("No font sheet handle found");

//         let texture_handle: Handle<Image> = find_first_handle_by_extension(&folder.handles, "png")
//             .expect("No texture handle found");

//         let font_sheet_data = font_sheet_data_assets
//             .get(font_sheet_data_handle.id())
//             .unwrap();

//         let mut texture_atlas = TextureAtlas::new_empty(
//             texture_handle,
//             Vec2 {
//                 x: font_sheet_data.width as f32,
//                 y: font_sheet_data.height as f32,
//             },
//         );

//         let mut glyphs = HashMap::with_capacity(font_sheet_data.characters.len());
//         for (id, glyph_data) in font_sheet_data.characters.iter() {
//             let index = texture_atlas.add_texture(glyph_data.rect);
//             glyphs.insert(
//                 *id,
//                 FontGlyph {
//                     index,
//                     color_less: false,
//                 },
//             );
//         }

//         let texture_atlas_handle = texture_atlases.add(texture_atlas);
//         let font_sheet_handle = font_sheets.add(FontSheet { glyphs });
//         font_assets.0.insert(
//             font.to_owned(),
//             FontAsset {
//                 texture_atlas: texture_atlas_handle,
//                 font_sheet: font_sheet_handle,
//             },
//         );
//     }

//     // Clean up unused resources
//     commands.remove_resource::<FontAssetsFolder>();
// }

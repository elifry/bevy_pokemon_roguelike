use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use itertools::Itertools;
use std::fs;
use std::path::Path;

use crate::graphics::anim_data::AnimData;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(key = "tiles.forest_path")]
    pub forest_path: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct PokemonAssets {
    #[asset(path = "images/pokemons", collection(mapped))]
    pub files: HashMap<String, UntypedHandle>,
}

#[derive(Resource, Debug)]
pub struct PokemonAnimationAssets {
    pub files: HashMap<String, PokemonAnimation>,
}

#[derive(Debug)]
pub struct PokemonAnimation {
    pub idle: Handle<TextureAtlas>,
}

impl FromWorld for PokemonAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let world_cell = world.cell();

        let asset_server = world_cell.get_resource::<AssetServer>().unwrap();
        let xml_asset = world_cell.get_resource::<Assets<AnimData>>().unwrap();

        let images = world_cell
            .get_resource::<PokemonAssets>()
            .expect("Failed to get Assets<PokemonAssets>");

        let mut texture_atlasses = world_cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .unwrap();

        let files: HashMap<String, PokemonAnimation> = images
            .files
            .iter()
            .sorted_by(|a, b| a.0.partial_cmp(b.0).unwrap())
            .group_by(|images| {
                let path = Path::new(images.0);
                let path = path.strip_prefix("images/pokemons").unwrap();
                let parent = path.parent().unwrap().to_str().unwrap();

                println!("{}", images.0);

                parent
            })
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(parent, mut group)| {
                println!("{}", parent);
                let anim_data_key = format!("{parent}/AnimData.xml");
                let anim_data_untyped = group
                    .find(|(path, _file)| path.contains(&anim_data_key))
                    .unwrap();

                let anim_data = anim_data_untyped.1.clone().typed::<AnimData>();

                let test = xml_asset.get(anim_data.id());

                println!("{:?}", test);

                let image_group: HashMap<&str, Handle<Image>> = group
                    .map(|(path, file)| {
                        let path = Path::new(path);

                        (path, file)
                    })
                    .filter(|(path, _file)| path.extension().unwrap() == "png")
                    .map(|(path, file)| {
                        let filename: &str = path.file_name().unwrap().to_str().unwrap();
                        println!("{}", filename);

                        let image = file.to_owned().typed::<Image>();

                        (filename, image)
                    })
                    .collect();

                let anim_data_key = format!("{parent}/AnimData.xml");

                let image = image_group.get("Idle-Anim.png").unwrap().to_owned();

                let texture_atlas =
                    TextureAtlas::from_grid(image, Vec2::splat(32.), 4, 8, None, None);

                let handle_texture_atlas = texture_atlasses.add(texture_atlas);

                let pokemon_animation = PokemonAnimation {
                    idle: handle_texture_atlas,
                };

                (parent.to_string(), pokemon_animation)
            })
            .collect();

        PokemonAnimationAssets { files }
    }
}

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use itertools::Itertools;
use std::path::Path;

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(key = "tiles.forest_path")]
    pub forest_path: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct PokemonAssets {
    #[asset(path = "images/pokemons", collection(typed, mapped))]
    pub files: HashMap<String, Handle<Image>>,
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
        let images = world_cell
            .get_resource::<PokemonAssets>()
            .expect("Failed to get Assets<PokemonAssets>");

        let mut texture_atlasses = world_cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .unwrap();

        let test: HashMap<String, PokemonAnimation> = images
            .files
            .iter()
            .sorted_by(|a, b| a.0.partial_cmp(b.0).unwrap())
            .group_by(|images| {
                let path = Path::new(images.0);
                let path = path.strip_prefix("images/pokemons").unwrap();
                let parent = path.parent().unwrap().to_str().unwrap();

                return parent;
            })
            .into_iter()
            .map(|(parent, group)| {
                println!("{}", parent);
                let group_map: HashMap<&str, Handle<Image>> = group
                    .into_iter()
                    .map(|(file, image)| {
                        let path = Path::new(file);
                        let filename: &str = path.file_name().unwrap().to_str().unwrap();
                        println!("{}", filename);
                        return (filename, image.to_owned());
                    })
                    .collect();
                let image = group_map.get("Idle-Anim.png").unwrap().to_owned();

                let texture_atlas =
                    TextureAtlas::from_grid(image, Vec2::splat(32.), 4, 8, None, None);

                let handle_texture_atlas = texture_atlasses.add(texture_atlas);

                let pokemon_animation = PokemonAnimation {
                    idle: handle_texture_atlas,
                };

                return (parent.to_string(), pokemon_animation);
            })
            .collect();

        return PokemonAnimationAssets { files: test };
    }
}

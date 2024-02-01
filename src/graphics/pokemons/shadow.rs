use bevy::{prelude::*};

use crate::{
    graphics::{
        anim_data::AnimData,
        assets::{AnimTextureType, PokemonAnimationAssets},
    },
    pieces::{FacingOrientation},
    pokemons::Pokemon,
};

use super::{
    pokemon_animator::get_pokemon_animator, AnimatorUpdatedEvent, PokemonAnimationState,
};

#[derive(Component, Default)]
pub enum PokemonShadow {
    Small, // Green
    #[default]
    Medium, // Red
    Big,   // Blue
}

#[allow(clippy::type_complexity)]
pub fn update_shadow_animator(
    mut query_child: Query<(Entity, &mut Handle<TextureAtlas>), With<PokemonShadow>>,
    query_parent: Query<(
        &FacingOrientation,
        &PokemonAnimationState,
        &Pokemon,
        &Children,
    )>,
    anim_data_assets: Res<Assets<AnimData>>,
    assets: Res<PokemonAnimationAssets>,
    mut commands: Commands,
    mut ev_animator_updated: EventReader<AnimatorUpdatedEvent>,
) {
    for ev in ev_animator_updated.read() {
        let Ok((facing_orientation, animation_state, pokemon, children)) = query_parent.get(ev.0)
        else {
            continue;
        };

        for child in children.iter() {
            let Ok((entity, mut texture_atlas)) = query_child.get_mut(*child) else {
                continue;
            };

            let pokemon_asset = assets.0.get(&pokemon.0).unwrap();
            let Some(shadow_animator) = get_pokemon_animator(
                &anim_data_assets,
                pokemon_asset,
                &animation_state.0,
                &AnimTextureType::Shadow,
                &facing_orientation.0,
            ) else {
                continue;
            };
            *texture_atlas = shadow_animator.texture_atlas.clone();
            // Resets PokemonShadow component to force change detection
            // Maybe use an event there ?
            commands
                .entity(entity)
                .insert((shadow_animator, PokemonShadow::default()));
        }
    }
}

/// Update the shadow image according to the shadow size
/// Ultimately this should be done with a shader material
/// But waiting for the implementation of https://github.com/bevyengine/bevy/pull/10845
pub fn update_pokemon_shadow_renderer(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(Entity, &Handle<TextureAtlas>, &PokemonShadow), Changed<PokemonShadow>>,
) {
    for (entity, texture_atlas_handle, shadow) in query.iter_mut() {
        // get the image from the texture handle
        if let Some(atlas) = atlases.get(texture_atlas_handle) {
            let image_handle = atlas.texture.clone();
            // get the image struct
            if let Some(image) = images.get(&image_handle) {
                // get raw image data
                let mut data = image.data.clone();

                // iterate over the image data
                for pixel in data.chunks_exact_mut(4) {
                    // set rgb parts of pixel based on palette

                    // pixel[0] = red / pixel[1] = green / pixel[2] = blue
                    // pixel[3] = alpha
                    let is_visible = match shadow {
                        PokemonShadow::Small => pixel[1] == 255,
                        PokemonShadow::Medium => pixel[0] == 255 || pixel[1] == 255,
                        PokemonShadow::Big => pixel[0] == 255 || pixel[1] == 255 || pixel[2] == 255,
                    };

                    if is_visible {
                        pixel[0] = 80;
                        pixel[1] = 80;
                        pixel[2] = 80;
                        pixel[3] = 180;
                    } else {
                        pixel[3] = 0;
                    }
                }

                // create a new image from the modified data
                let new_image = Image {
                    data,
                    ..image.clone()
                };

                // add the image to the assets, to get a handle
                let new_image_handle = images.add(new_image);

                // create a new texture atlas from the new texture
                let mut new_texture_atlas = TextureAtlas::new_empty(new_image_handle, atlas.size);
                new_texture_atlas.textures = atlas.textures.clone();
                let new_atlas_handle = atlases.add(new_texture_atlas);

                // replace the texture atlas handle on the entity
                commands.entity(entity).insert(new_atlas_handle);
            }
        }
    }
}

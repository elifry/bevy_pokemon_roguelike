use bevy::prelude::*;

use crate::{pieces::FacingOrientation, pokemons::Pokemon};

use super::{pokemon_animator::get_pokemon_animator, AnimatorUpdatedEvent, PokemonAnimationState};

#[derive(Component, Default)]
pub enum PokemonShadow {
    Small, // Green
    #[default]
    Medium, // Red
    Big,   // Blue
}

// #[allow(clippy::type_complexity)]
// pub fn update_shadow_animator(
//     mut query_child: Query<(Entity, &mut TextureAtlas, &mut Handle<Image>), With<PokemonShadow>>,
//     query_parent: Query<(
//         &FacingOrientation,
//         &PokemonAnimationState,
//         &Pokemon,
//         &Children,
//     )>,
//     anim_data_assets: Res<Assets<AnimData>>,
//     assets: Res<PokemonAnimationAssets>,
//     mut commands: Commands,
//     mut ev_animator_updated: EventReader<AnimatorUpdatedEvent>,
// ) {
//     for ev in ev_animator_updated.read() {
//         let Ok((facing_orientation, animation_state, pokemon, children)) = query_parent.get(ev.0)
//         else {
//             continue;
//         };

//         for child in children.iter() {
//             let Ok((entity, mut texture_atlas, mut texture)) = query_child.get_mut(*child) else {
//                 continue;
//             };

//             let pokemon_asset = assets.0.get(pokemon).unwrap();
//             let Some(shadow_animator) = get_pokemon_animator(
//                 &anim_data_assets,
//                 pokemon_asset,
//                 &animation_state.0,
//                 &AnimTextureType::Shadow,
//                 &facing_orientation.0,
//             ) else {
//                 continue;
//             };
//             texture_atlas.layout = shadow_animator.atlas_layout.clone();
//             *texture = shadow_animator.texture.clone();
//             // Resets PokemonShadow component to force change detection
//             // Maybe use an event there ?
//             commands
//                 .entity(entity)
//                 .insert((shadow_animator, PokemonShadow::default()));
//         }
//     }
// }

// /// Update the shadow image according to the shadow size
// /// Ultimately this should be done with a shader material
// /// But waiting for the implementation of https://github.com/bevyengine/bevy/pull/10845
// pub fn update_pokemon_shadow_renderer(
//     mut commands: Commands,
//     mut atlases: ResMut<Assets<TextureAtlasLayout>>,
//     mut images: ResMut<Assets<Image>>,
//     mut query: Query<(Entity, &Handle<Image>, &PokemonShadow), Changed<PokemonShadow>>,
// ) {
//     for (entity, image_handle, shadow) in query.iter_mut() {
//         // get the image struct
//         let Some(image) = images.get(image_handle) else {
//             continue;
//         };
//         // get raw image data
//         let mut data = image.data.clone();

//         // iterate over the image data
//         for pixel in data.chunks_exact_mut(4) {
//             // set rgb parts of pixel based on palette

//             // pixel[0] = red / pixel[1] = green / pixel[2] = blue
//             // pixel[3] = alpha
//             let is_visible = match shadow {
//                 PokemonShadow::Small => pixel[1] == 255,
//                 PokemonShadow::Medium => pixel[0] == 255 || pixel[1] == 255,
//                 PokemonShadow::Big => pixel[0] == 255 || pixel[1] == 255 || pixel[2] == 255,
//             };

//             if is_visible {
//                 pixel[0] = 80;
//                 pixel[1] = 80;
//                 pixel[2] = 80;
//                 pixel[3] = 180;
//             } else {
//                 pixel[3] = 0;
//             }
//         }

//         // create a new image from the modified data
//         let new_image = Image {
//             data,
//             ..image.clone()
//         };

//         // add the image to the assets, to get a handle
//         let new_image_handle = images.add(new_image);

//         // replace the texture handle on the entity
//         commands.entity(entity).insert(new_image_handle);
//     }
// }

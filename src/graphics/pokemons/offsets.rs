use bevy::{gizmos, prelude::*};

use crate::{
    graphics::{
        anim_data::AnimData,
        animations::AnimationFrameChangedEvent,
        assets::{AnimTextureType, PokemonAnimationAssets},
    },
    pieces::FacingOrientation,
    pokemons::Pokemon,
};

use super::{pokemon_animator::get_pokemon_animator, AnimatorUpdatedEvent, PokemonAnimationState};

#[derive(Component, Default)]
pub struct PokemonOffsets {
    body: Vec2,  // Green
    head: Vec2,  // Black
    right: Vec2, // Blue
    left: Vec2,  // Red
}

#[allow(clippy::type_complexity)]
pub fn update_offsets_animator(
    mut query_child: Query<(Entity, &mut Handle<TextureAtlas>), With<PokemonOffsets>>,
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
            let Some(offsets_animator) = get_pokemon_animator(
                &anim_data_assets,
                pokemon_asset,
                &animation_state.0,
                &AnimTextureType::Offsets,
                &facing_orientation.0,
            ) else {
                continue;
            };
            *texture_atlas = offsets_animator.texture_atlas.clone();
            commands.entity(entity).insert(offsets_animator);
        }
    }
}

/// Update the [`PokemonOffsets`] based on its current texture each new animation frame
pub fn update_offsets(
    mut query_offsets: Query<(&mut PokemonOffsets, &Handle<TextureAtlas>)>,
    mut ev_frame_changed: EventReader<AnimationFrameChangedEvent>,
    atlases: ResMut<Assets<TextureAtlas>>,
    images: ResMut<Assets<Image>>,
) {
    for ev in ev_frame_changed.read() {
        let Ok((mut offsets, texture_atlas_handle)) = query_offsets.get_mut(ev.entity) else {
            continue;
        };
        let Some(atlas) = atlases.get(texture_atlas_handle) else {
            continue;
        };
        let image_handle = atlas.texture.clone();
        // get the image struct
        let Some(image) = images.get(&image_handle) else {
            continue;
        };

        // Get the current texture
        let Some(texture) = atlas.textures.get(ev.frame.atlas_index) else {
            continue;
        };

        let atlas_image_width = image.texture_descriptor.size.width;

        // Calculate the number of bytes per row (assuming RGBA format, hence * 4)
        let bytes_per_row = atlas_image_width as usize * 4;

        for y in (texture.min.y as i32)..=(texture.max.y as i32) {
            for x in (texture.min.x as i32)..=(texture.max.x as i32) {
                // Calculate the starting index of the pixel in the linear array
                let start_index = (y as usize * bytes_per_row) + (x as usize * 4);

                // Access individual color components
                let r = image.data[start_index];
                let g = image.data[start_index + 1];
                let b = image.data[start_index + 2];
                let a = image.data[start_index + 3];

                let real_x = x as f32 - texture.min.x;
                let real_y = y as f32 - texture.min.y;
                if g == 255 {
                    warn!("fond body at {}, {}", real_x, real_y);
                    offsets.body = Vec2::new(real_x, real_y);
                }

                // Now you have the RGBA values for the pixel at (x, y)
                // You can process them as needed
            }
        }
    }
}

pub fn debug_offsets(
    query_offsets: Query<(&mut PokemonOffsets, &GlobalTransform, &Parent)>,
    query_parent: Query<(&PokemonAnimationState, &Pokemon)>,
    anim_data_assets: Res<Assets<AnimData>>,
    pokemon_animation_assets: ResMut<PokemonAnimationAssets>,
    mut gizmos: Gizmos,
) {
    for (offsets, global_transform, parent) in query_offsets.iter() {
        let Ok((animation_state, pokemon)) = query_parent.get(**parent) else {
            continue;
        };
        let Some(pokemon_animation) = pokemon_animation_assets.0.get(&pokemon.0) else {
            continue;
        };
        let Some(anim_data) = anim_data_assets.get(&pokemon_animation.anim_data) else {
            continue;
        };
        let anim_info = anim_data.get(animation_state.0);
        // Extract the base translation as Vec2 directly from the global_transform's x and y components
        let base_translation = Vec2::new(
            global_transform.translation().x,
            global_transform.translation().y,
        );

        // Calculate the half tile size vector for adjustment
        let half_tile_size = anim_info.tile_size() / 2.0;

        // Calculate the offset vector based on provided offsets and adjustments
        // Notice the subtraction of `offsets.body.y` from `anim_info.tile_size().y` for correct y-axis adjustment
        let offset_vector = Vec2::new(offsets.body.x, anim_info.tile_size().y - offsets.body.y);

        // Combine the base translation with adjustments and offsets
        // This single line combines all calculations into a clearer and more concise expression
        let position = base_translation - half_tile_size + offset_vector;
        gizmos.circle_2d(position + Vec2::new(0.5, 0.5), 1., Color::GREEN);
    }
}

use bevy::prelude::*;

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
    pub body: Vec2,  // Green
    pub head: Vec2,  // Black
    pub right: Vec2, // Blue
    pub left: Vec2,  // Red
}

//
#[derive(Component, Default)]
pub struct PokemonHeadOffset;

#[derive(Component, Default)]
pub struct PokemonBodyOffset;

#[derive(Component, Default)]
pub struct PokemonLeftOffset;

#[derive(Component, Default)]
pub struct PokemonRightOffset;

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

            let pokemon_asset = assets.0.get(pokemon).unwrap();
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
    mut query_offsets: Query<(&mut PokemonOffsets, &Handle<TextureAtlas>, &Parent)>,
    query_parent: Query<(&Pokemon, &PokemonAnimationState)>,
    mut ev_frame_changed: EventReader<AnimationFrameChangedEvent>,
    atlases: ResMut<Assets<TextureAtlas>>,
    images: ResMut<Assets<Image>>,
    anim_data_assets: Res<Assets<AnimData>>,
    pokemon_animation_assets: ResMut<PokemonAnimationAssets>,
) {
    for ev in ev_frame_changed.read() {
        let Ok((mut offsets, texture_atlas_handle, parent)) = query_offsets.get_mut(ev.entity)
        else {
            continue;
        };

        let Ok((pokemon, animation_state)) = query_parent.get(parent.get()) else {
            continue;
        };

        let Some(pokemon_animation) = pokemon_animation_assets.0.get(pokemon) else {
            continue;
        };

        let Some(anim_data) = anim_data_assets.get(&pokemon_animation.anim_data) else {
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

        let anim_info = anim_data.get(animation_state.0);

        let tile_size = anim_info.tile_size();

        let atlas_image_width = image.texture_descriptor.size.width;

        // Calculate the number of bytes per row (assuming RGBA format, hence * 4)
        let bytes_per_row = atlas_image_width as usize * 4;

        for y in (texture.min.y as i32)..=(texture.max.y as i32) {
            for x in (texture.min.x as i32)..=(texture.max.x as i32) {
                // Calculate the starting index of the pixel in the linear array
                let start_index = (y as usize * bytes_per_row) + (x as usize * 4);

                // Access individual color components
                let red = image.data[start_index];
                let green = image.data[start_index + 1];
                let blue = image.data[start_index + 2];
                let alpha = image.data[start_index + 3];

                let real_x = x as f32 - texture.min.x;
                let real_y = y as f32 - texture.min.y;

                if red == 0 && green == 0 && blue == 0 && alpha == 255 {
                    offsets.head = calculate_offset(real_x, real_y, tile_size);
                } else if green == 255 {
                    offsets.body = calculate_offset(real_x, real_y, tile_size);
                } else if red == 255 {
                    offsets.left = calculate_offset(real_x, real_y, tile_size);
                } else if blue == 255 {
                    offsets.right = calculate_offset(real_x, real_y, tile_size);
                }
            }
        }
    }
}

fn calculate_offset(real_x: f32, real_y: f32, tile_size: Vec2) -> Vec2 {
    let half_tile_size = tile_size / 2.;
    let coordinates = Vec2::new(real_x, tile_size.y - real_y);
    coordinates - Vec2::new(half_tile_size.x, half_tile_size.y)
}

pub fn update_head_offset(
    mut query_head_offset: Query<(&Parent, &mut Transform), With<PokemonHeadOffset>>,
    query_parent: Query<&Children, With<Pokemon>>,
    query_offsets: Query<&mut PokemonOffsets>,
) {
    for (parent, mut transform) in query_head_offset.iter_mut() {
        let Ok(children) = query_parent.get(parent.get()) else {
            continue;
        };
        let Some(offsets) = children
            .iter()
            .filter_map(|&child| query_offsets.get(child).ok())
            .next()
        else {
            continue;
        };

        transform.translation = Vec3::new(offsets.head.x, offsets.head.y, 1.);
    }
}

pub fn debug_offsets(
    query_offsets: Query<(&mut PokemonOffsets, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (offsets, global_transform) in query_offsets.iter() {
        // Extract the base translation as Vec2 directly from the global_transform's x and y components
        let base_translation = Vec2::new(
            global_transform.translation().x,
            global_transform.translation().y,
        );

        // Calculate the offset vector based on provided offsets and adjustments
        let body_position = base_translation + offsets.body;
        gizmos.circle_2d(body_position + Vec2::new(0.5, 0.5), 1., Color::GREEN);

        let head_position = base_translation + offsets.head;
        gizmos.circle_2d(head_position + Vec2::new(0.5, 0.5), 1., Color::BLACK);

        let right_position = base_translation + offsets.right;
        gizmos.circle_2d(right_position + Vec2::new(0.5, 0.5), 1., Color::BLUE);

        let left_position = base_translation + offsets.left;
        gizmos.circle_2d(left_position + Vec2::new(0.5, 0.5), 1., Color::RED);
    }
}

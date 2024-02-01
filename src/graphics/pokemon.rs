use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    constants::GAME_SPEED,
    graphics::animations::Animator,
    map::Position,
    pieces::{FacingOrientation, Orientation},
    pokemons::Pokemon,
    GameState,
};

use super::{
    anim_data::{AnimData, AnimKey},
    animations::{AnimationFrame, AnimationIndices},
    assets::{AnimTextureType, PokemonAnimation, PokemonAnimationAssets},
    POKEMON_Z, SHADOW_POKEMON_Z,
};

pub struct PokemonPlugin;

impl Plugin for PokemonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_pokemon_renderer,).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                update_shadow_animator,
                apply_deferred,
                update_pokemon_shadow_renderer,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Default)]
pub enum PokemonShadow {
    Small, // Green
    #[default]
    Medium, // Red
    Big,   // Blue
}

#[derive(Component, Default)]
pub struct PokemonAnimationState(pub AnimKey);

#[allow(clippy::type_complexity)]
pub fn update_animator(
    mut query: Query<
        (
            Entity,
            &FacingOrientation,
            &PokemonAnimationState,
            &Pokemon,
            &mut Handle<TextureAtlas>,
        ),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    anim_data_assets: Res<Assets<AnimData>>,
    assets: Res<PokemonAnimationAssets>,
    mut commands: Commands,
) {
    for (entity, facing_orientation, animation_state, pokemon, mut texture_atlas) in
        query.iter_mut()
    {
        let pokemon_asset = assets.0.get(&pokemon.0).unwrap();
        let Some(animator) = get_pokemon_animator(
            &anim_data_assets,
            pokemon_asset,
            &animation_state.0,
            &AnimTextureType::Anim,
            &facing_orientation.0,
        ) else {
            continue;
        };
        *texture_atlas = animator.texture_atlas.clone();
        commands.entity(entity).insert(animator);
    }
}

#[allow(clippy::type_complexity)]
fn update_shadow_animator(
    mut query: Query<(Entity, &Parent, &mut Handle<TextureAtlas>), With<PokemonShadow>>,
    query_parent: Query<
        (&FacingOrientation, &PokemonAnimationState, &Pokemon),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    anim_data_assets: Res<Assets<AnimData>>,
    assets: Res<PokemonAnimationAssets>,
    mut commands: Commands,
) {
    for (entity, parent, mut texture_atlas) in query.iter_mut() {
        let Ok((facing_orientation, animation_state, pokemon)) = query_parent.get(parent.get())
        else {
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
        commands
            .entity(entity)
            .insert((shadow_animator, PokemonShadow::default()));

        break;
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

fn spawn_pokemon_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    query: Query<(Entity, &Position, &Pokemon), Added<Pokemon>>,
) {
    let default_state = AnimKey::Idle;
    for (entity, position, pokemon) in query.iter() {
        let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

        let v = super::get_world_position(&position.0, POKEMON_Z);
        let sprite = TextureAtlasSprite {
            index: 0,
            anchor: Anchor::Center,
            ..default()
        };
        let Some(anim_texture_atlas) = pokemon_animation
            .textures
            .get(&default_state)
            .and_then(|t| t.get(&AnimTextureType::Anim))
        else {
            continue;
        };

        commands
            .entity(entity)
            .insert((
                PokemonAnimationState(default_state),
                SpriteSheetBundle {
                    texture_atlas: anim_texture_atlas.clone(),
                    sprite,
                    transform: Transform::from_translation(v),
                    ..default()
                },
            ))
            .with_children(|parent| {
                let Some(shadow_texture_atlas) = pokemon_animation
                    .textures
                    .get(&default_state)
                    .and_then(|t| t.get(&AnimTextureType::Shadow))
                else {
                    return;
                };

                let sprite = TextureAtlasSprite {
                    index: 0,
                    anchor: Anchor::Center,
                    ..default()
                };

                parent.spawn((
                    PokemonShadow::default(),
                    SpriteSheetBundle {
                        texture_atlas: shadow_texture_atlas.clone(),
                        sprite,
                        transform: Transform::from_xyz(0., 0., SHADOW_POKEMON_Z),
                        ..default()
                    },
                ));
            });
    }
}

fn get_pokemon_animator(
    anim_data_assets: &Res<'_, Assets<AnimData>>,
    pokemon_animation: &PokemonAnimation,
    anim_key: &AnimKey,
    anim_texture_type: &AnimTextureType,
    orientation: &Orientation,
) -> Option<Animator> {
    let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
    let anim_info = anim_data.get(*anim_key);

    let is_loop = matches!(anim_key, AnimKey::Idle);

    let animation_indices = AnimationIndices::from_animation(orientation, &anim_info);

    let frames = anim_info
        .value()
        .durations
        .duration
        .iter()
        .enumerate()
        .map(|(index, duration)| AnimationFrame {
            duration: Duration::from_millis(
                ((duration.value * 22) as f32 / GAME_SPEED).floor() as u64
            ),
            atlas_index: animation_indices.first + index,
        })
        .collect::<Vec<_>>();

    let Some(texture_atlas) = pokemon_animation
        .textures
        .get(anim_key)
        .and_then(|t| t.get(anim_texture_type))
    else {
        return None;
    };

    Some(Animator::new(texture_atlas.clone(), frames, is_loop))
}

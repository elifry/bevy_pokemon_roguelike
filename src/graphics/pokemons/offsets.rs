

use bevy::{prelude::*};

use crate::{
    graphics::{
        anim_data::{AnimData},
        assets::{AnimTextureType, PokemonAnimationAssets},
    },
    pieces::{FacingOrientation},
    pokemons::Pokemon,
};

use super::{
    pokemon_animator::get_pokemon_animator, AnimatorUpdatedEvent, PokemonAnimationState,
};

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

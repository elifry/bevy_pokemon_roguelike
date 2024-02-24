pub mod offsets;
mod pokemon_animator;
mod shadow;

use bevy::prelude::*;
use char_animation::{anim_key::AnimKey, CharAnimation};

use crate::{map::Position, pieces::FacingOrientation, pokemons::Pokemon, GameState};

use self::{
    offsets::{
        debug_offsets, update_body_offset, update_head_offset, PokemonBodyOffset,
        PokemonHeadOffset, PokemonOffsets,
    },
    pokemon_animator::get_pokemon_animator,
};

use super::{
    action_animations::ActionAnimationSet,
    assets::pokemon_chara_assets::{PokemonCharaAssets, PokemonCharaAssetsPlugin},
    POKEMON_Z, SHADOW_POKEMON_Z,
};

pub struct PokemonPlugin;

// TODO: Create plugin for sub systems
impl Plugin for PokemonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimatorUpdatedEvent>()
            .add_systems(
                Update,
                (spawn_pokemon_renderer,).run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_animator,
                    // update_shadow_animator,
                    // update_offsets_animator,
                    update_head_offset,
                    update_body_offset,
                    // update_pokemon_shadow_renderer,
                )
                    .chain()
                    .in_set(ActionAnimationSet::Animator),
            );
        // .add_systems(Update, update_offsets.after(GamePlayingSet::LateLogics));
        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, (debug_offsets).run_if(in_state(GameState::Playing)));
        }
    }
}

#[derive(Event, Debug)]
pub struct AnimatorUpdatedEvent(pub Entity);

#[derive(Component, Default)]
pub struct PokemonAnimationState(pub AnimKey);

#[allow(clippy::type_complexity)]
fn update_animator(
    mut query: Query<
        (
            Entity,
            &FacingOrientation,
            &PokemonAnimationState,
            &Pokemon,
            &mut TextureAtlas,
            &mut Handle<Image>,
        ),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    char_animation_assets: Res<Assets<CharAnimation>>,
    pokemon_char_assets: Res<PokemonCharaAssets>,
    mut ev_animator_updated: EventWriter<AnimatorUpdatedEvent>,
    mut commands: Commands,
) {
    for (entity, facing_orientation, animation_state, pokemon, mut texture_atlas, mut texture) in
        query.iter_mut()
    {
        // TODO: replace by the real id of the pokemon
        let char_animation_handle = pokemon_char_assets.0.get("0001").unwrap();
        let Some(animator) = get_pokemon_animator(
            &char_animation_assets,
            char_animation_handle,
            &animation_state.0,
            &facing_orientation.0,
        ) else {
            continue;
        };
        texture_atlas.layout = animator.atlas_layout.clone();
        *texture = animator.texture.clone();
        commands.entity(entity).insert(animator);
        ev_animator_updated.send(AnimatorUpdatedEvent(entity));
    }
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    char_animation_assets: Res<Assets<CharAnimation>>,
    pokemon_char_assets: Res<PokemonCharaAssets>,
    query: Query<(Entity, &Position, &Pokemon), Added<Pokemon>>,
) {
    let default_state = AnimKey::Idle;
    for (entity, position, pokemon) in query.iter() {
        let pokemon_animation_handle = pokemon_char_assets.0.get("0001").unwrap();
        let pokemon_char_animation = char_animation_assets.get(pokemon_animation_handle).unwrap();
        let animation_data = pokemon_char_animation.anim.get(&default_state).unwrap();

        let v = super::get_world_position(&position.0, POKEMON_Z);

        let atlas = TextureAtlas {
            index: 0,
            layout: animation_data.atlas_layout.clone(),
        };

        commands
            .entity(entity)
            .insert((
                PokemonAnimationState(default_state),
                SpriteSheetBundle {
                    atlas,
                    texture: animation_data.texture.clone(),
                    transform: Transform::from_translation(v),
                    ..default()
                },
            ))
            // .with_children(|parent| {
            //     // Shadow
            //     let Some(shadow_texture_atlas) = pokemon_animation
            //         .textures
            //         .get(&default_state)
            //         .and_then(|t| t.get(&AnimTextureType::Shadow))
            //     else {
            //         return;
            //     };
            //     let shadow_atlas = TextureAtlas {
            //         index: 0,
            //         layout: shadow_texture_atlas.0.clone(),
            //         ..default()
            //     };
            //     parent.spawn((
            //         Name::new("Shadow"),
            //         PokemonShadow::default(),
            //         SpriteSheetBundle {
            //             atlas: shadow_atlas,
            //             texture: shadow_texture_atlas.1.clone(),
            //             transform: Transform::from_xyz(0., 0., SHADOW_POKEMON_Z),
            //             ..default()
            //         },
            //     ));
            // })
            .with_children(|parent| {
                parent.spawn((
                    Name::new("HeadOffset"),
                    PokemonHeadOffset,
                    SpatialBundle::default(),
                ));
            })
            .with_children(|parent| {
                parent.spawn((
                    Name::new("BodyOffset"),
                    PokemonBodyOffset,
                    SpatialBundle::default(),
                ));
            });
        // .with_children(|parent| {
        //     // Offsets
        //     let Some(offsets_texture_atlas) = pokemon_animation
        //         .textures
        //         .get(&default_state)
        //         .and_then(|t| t.get(&AnimTextureType::Offsets))
        //     else {
        //         warn!("unable to load offsets for {:?}", entity);
        //         return;
        //     };

        //     let offsets_atlas = TextureAtlas {
        //         index: 0,
        //         layout: offsets_texture_atlas.0.clone(),
        //         ..default()
        //     };

        //     parent.spawn((
        //         Name::new("Offsets"),
        //         PokemonOffsets::default(),
        //         SpriteSheetBundle {
        //             atlas: offsets_atlas,
        //             texture: offsets_texture_atlas.1.clone(),
        //             transform: Transform::from_xyz(0., 0., POKEMON_Z + 1.),
        //             visibility: Visibility::Hidden,
        //             ..default()
        //         },
        //     ));
        // });
    }
}

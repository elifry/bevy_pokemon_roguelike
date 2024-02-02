mod offsets;
mod pokemon_animator;
mod shadow;

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    map::Position, pieces::FacingOrientation, pokemons::Pokemon, GamePlayingSet, GameState,
};

use self::{
    offsets::{debug_offsets, update_offsets, update_offsets_animator, PokemonOffsets},
    pokemon_animator::get_pokemon_animator,
    shadow::{update_pokemon_shadow_renderer, update_shadow_animator, PokemonShadow},
};

use super::{
    action_animation::ActionAnimationSet,
    anim_data::{AnimData, AnimKey},
    assets::{AnimTextureType, PokemonAnimationAssets},
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
                    update_shadow_animator,
                    update_offsets_animator,
                    apply_deferred,
                    update_pokemon_shadow_renderer,
                )
                    .chain()
                    .in_set(ActionAnimationSet::Animator),
            )
            .add_systems(Update, update_offsets.after(GamePlayingSet::LateLogics));
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
            &mut Handle<TextureAtlas>,
        ),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    anim_data_assets: Res<Assets<AnimData>>,
    assets: Res<PokemonAnimationAssets>,
    mut ev_animator_updated: EventWriter<AnimatorUpdatedEvent>,
    mut commands: Commands,
) {
    for (entity, facing_orientation, animation_state, pokemon, mut texture_atlas) in
        query.iter_mut()
    {
        let pokemon_asset = assets.0.get(pokemon).unwrap();
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
        ev_animator_updated.send(AnimatorUpdatedEvent(entity));
    }
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    query: Query<(Entity, &Position, &Pokemon), Added<Pokemon>>,
) {
    let default_state = AnimKey::Idle;
    for (entity, position, pokemon) in query.iter() {
        let pokemon_animation = assets.0.get(pokemon).unwrap();

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
                // Shadow
                let Some(shadow_texture_atlas) = pokemon_animation
                    .textures
                    .get(&default_state)
                    .and_then(|t| t.get(&AnimTextureType::Shadow))
                else {
                    return;
                };

                let shadow_sprite = TextureAtlasSprite {
                    index: 0,
                    anchor: Anchor::Center,
                    ..default()
                };

                parent.spawn((
                    PokemonShadow::default(),
                    SpriteSheetBundle {
                        texture_atlas: shadow_texture_atlas.clone(),
                        sprite: shadow_sprite,
                        transform: Transform::from_xyz(0., 0., SHADOW_POKEMON_Z),
                        ..default()
                    },
                ));
            })
            .with_children(|parent| {
                // Offsets
                let Some(offsets_texture_atlas) = pokemon_animation
                    .textures
                    .get(&default_state)
                    .and_then(|t| t.get(&AnimTextureType::Offsets))
                else {
                    warn!("unable to load offsets for {:?}", entity);
                    return;
                };

                let offsets_sprite = TextureAtlasSprite {
                    index: 0,
                    anchor: Anchor::Center,
                    ..default()
                };

                parent.spawn((
                    PokemonOffsets::default(),
                    SpriteSheetBundle {
                        texture_atlas: offsets_texture_atlas.clone(),
                        sprite: offsets_sprite,
                        transform: Transform::from_xyz(0., 0., POKEMON_Z + 1.),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                ));
            });
    }
}

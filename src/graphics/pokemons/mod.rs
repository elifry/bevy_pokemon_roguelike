pub mod offsets;
mod pokemon_animator;
mod shadow;

use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};
use char_animation::{anim_key::AnimKey, CharAnimation};

use crate::{
    actions::{walk_action::WalkAction, RunningAction},
    constants::GAME_SPEED,
    graphics::{
        animations::{AnimationFrame, AnimationIndices, Animator},
        get_world_position, POKEMON_Z,
    },
    map::Position,
    pieces::{FacingOrientation, Piece},
    pokemons::Pokemon,
    turn::TurnOrder,
    GamePlayingSet, GameState,
};

use self::{
    offsets::{
        debug_offsets, update_body_offset, update_head_offset, update_offsets, PokemonBodyOffset,
        PokemonHeadOffset,
    },
    pokemon_animator::get_pokemon_animator,
    shadow::{spawn_shadow_renderer, update_shadow_offsets, PokemonShadow},
};

use super::{
    action_animations::ActionAnimationSet, assets::pokemon_chara_assets::PokemonCharaAssets,
};

/// A wrapper component for TextureAtlas to make it compatible with Bevy 0.15
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PokemonTextureAtlas(pub TextureAtlas);

impl Default for PokemonTextureAtlas {
    fn default() -> Self {
        Self(TextureAtlas::default())
    }
}

/// A wrapper component for Handle<CharAnimation> to make it compatible with Bevy 0.15
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PokemonCharAnimationHandle(pub Handle<CharAnimation>);

impl Default for PokemonCharAnimationHandle {
    fn default() -> Self {
        Self(Handle::default())
    }
}

/// A wrapper component for Handle<Image> to make it compatible with Bevy 0.15
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PokemonImageHandle(pub Handle<Image>);

impl Default for PokemonImageHandle {
    fn default() -> Self {
        Self(Handle::default())
    }
}

pub struct PokemonPlugin;

// TODO: Create plugin for sub systems
impl Plugin for PokemonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PokemonAnimationState>()
            .register_type::<PokemonTextureAtlas>()
            .register_type::<PokemonImageHandle>()
            .register_type::<PokemonCharAnimationHandle>()
            .add_event::<AnimatorUpdatedEvent>()
            .add_systems(
                Update,
                (spawn_pokemon_renderer, spawn_shadow_renderer)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_animator,
                    animate_pokemon_animator,
                    // update_shadow_animator,
                    // update_offsets_animator,
                    update_head_offset,
                    update_body_offset,
                    // update_pokemon_shadow_renderer,
                )
                    .chain()
                    .in_set(ActionAnimationSet::Animator),
            )
            .add_systems(
                Update,
                (update_offsets, update_shadow_offsets).after(GamePlayingSet::LateLogics),
            );
        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, (debug_offsets).run_if(in_state(GameState::Playing)));
        }
    }
}

#[derive(Event, Debug)]
pub struct AnimatorUpdatedEvent(pub Entity);

#[derive(Component, Default, Reflect)]
pub struct PokemonAnimationState(pub AnimKey);

#[allow(clippy::type_complexity)]
fn update_animator(
    mut query: Query<
        (
            Entity,
            &FacingOrientation,
            &PokemonAnimationState,
            &PokemonCharAnimationHandle,
            &mut Sprite,
            &mut Animator,
        ),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    char_animation_assets: Res<Assets<CharAnimation>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut ev_animator_updated: EventWriter<AnimatorUpdatedEvent>,
    mut commands: Commands,
) {
    for (
        entity,
        facing_orientation,
        animation_state,
        char_animation_handle,
        mut sprite,
        mut animator,
    ) in query.iter_mut()
    {
        let Some(new_animator) = get_pokemon_animator(
            &char_animation_assets,
            char_animation_handle,
            &animation_state.0,
            &facing_orientation.0,
        ) else {
            continue;
        };

        // Update the animator component with the new animation data
        *animator = new_animator;

        // Update the sprite with new animation data
        let layout = atlas_layouts
            .get(&animator.atlas_layout)
            .expect("Pokemon atlas layout not loaded");
        let urect = layout.textures[animator.current_frame]; // Use current frame
        let rect = Rect::new(
            urect.min.x as f32,
            urect.min.y as f32,
            urect.max.x as f32,
            urect.max.y as f32,
        );

        let sprite_size = Vec2::new(
            (urect.max.x - urect.min.x) as f32,
            (urect.max.y - urect.min.y) as f32,
        );

        sprite.image = animator.texture.clone();
        sprite.rect = Some(rect);
        sprite.custom_size = Some(sprite_size);

        ev_animator_updated.send(AnimatorUpdatedEvent(entity));
    }
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    char_animation_assets: Res<Assets<CharAnimation>>,
    pokemon_char_assets: Res<PokemonCharaAssets>,
    query: Query<(Entity, &Position, &Pokemon, &FacingOrientation), Added<Pokemon>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    let default_state = AnimKey::Idle;
    for (entity, position, pokemon, orientation) in query.iter() {
        let pokemon_animation_handle = pokemon_char_assets.0.get(&pokemon.id).unwrap();
        let pokemon_char_animation = char_animation_assets.get(pokemon_animation_handle).unwrap();
        let animation_data = pokemon_char_animation.anim.get(&default_state).unwrap();

        let char_animation_offsets = &animation_data.offsets.get(&orientation.0).unwrap()[0];

        let v = super::get_world_position(&position.0, POKEMON_Z);

        // Get the atlas layout and calculate the sprite rect
        let layout = atlas_layouts
            .get(&animation_data.atlas_layout)
            .expect("Pokemon atlas layout not loaded");
        let urect = layout.textures[0]; // Use first frame for now
        let rect = Rect::new(
            urect.min.x as f32,
            urect.min.y as f32,
            urect.max.x as f32,
            urect.max.y as f32,
        );

        // Calculate proper sprite size from the texture rectangle
        let sprite_size = Vec2::new(
            (urect.max.x - urect.min.x) as f32,
            (urect.max.y - urect.min.y) as f32,
        );

        // Create the animator for this animation
        let animator = get_pokemon_animator(
            &char_animation_assets,
            &pokemon_animation_handle,
            &default_state,
            &orientation.0,
        )
        .expect("Failed to create animator");

        let mut entity_commands = commands.entity(entity);
        entity_commands.insert((
            PokemonAnimationState(default_state),
            char_animation_offsets.clone(),
            Sprite {
                custom_size: Some(sprite_size), // Pokemon sprite size
                image: animation_data.texture.clone(),
                rect: Some(rect),
                ..Default::default()
            },
            Transform::from_translation(v),
            Visibility::default(),
            InheritedVisibility::default(),
            animator, // Add the animator component back
        ));
        entity_commands.insert(PokemonCharAnimationHandle(pokemon_animation_handle.clone()));

        entity_commands
            .with_children(|parent| {
                // Shadow
                parent.spawn((Name::new("Shadow"), PokemonShadow::default()));
            })
            .with_children(|parent| {
                parent.spawn((
                    Name::new("HeadOffset"),
                    PokemonHeadOffset,
                    Transform::default(),
                    Visibility::default(),
                ));
            })
            .with_children(|parent| {
                parent.spawn((
                    Name::new("BodyOffset"),
                    PokemonBodyOffset,
                    Transform::default(),
                    Visibility::default(),
                ));
            });
    }
}

fn animate_pokemon_animator(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Animator, &mut Sprite)>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (entity, mut animator, mut sprite) in query.iter_mut() {
        animator.timer.tick(time.delta());

        if !animator.timer.finished() {
            continue;
        }

        if !animator.is_loop && animator.current_frame >= animator.frames.len() - 1 {
            // Animation is finished
            continue;
        }

        let Some(frame) = animator.frames.get(animator.current_frame).cloned() else {
            warn!("animation frame not found for entity {:?}", entity);
            continue;
        };

        // Update the sprite to show the current frame
        let layout = atlas_layouts
            .get(&animator.atlas_layout)
            .expect("Pokemon atlas layout not loaded");
        let urect = layout.textures[frame.atlas_index];
        let rect = Rect::new(
            urect.min.x as f32,
            urect.min.y as f32,
            urect.max.x as f32,
            urect.max.y as f32,
        );

        let sprite_size = Vec2::new(
            (urect.max.x - urect.min.x) as f32,
            (urect.max.y - urect.min.y) as f32,
        );

        sprite.rect = Some(rect);
        sprite.custom_size = Some(sprite_size);

        // Set timer for next frame
        animator.timer.set_duration(frame.duration);
        animator.timer.reset();

        // Move to next frame
        animator.current_frame = if animator.current_frame + 1 < animator.frames.len() {
            animator.current_frame + 1
        } else if animator.is_loop {
            0
        } else {
            animator.current_frame + 1
        };
    }
}

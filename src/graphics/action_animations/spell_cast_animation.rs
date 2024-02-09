use bevy::prelude::*;

use crate::{
    actions::{damage_action::DamageAction, spell_action::SpellAction, RunningAction},
    graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState},
    spells::SpellCast,
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct SpellCastAnimationPlugin;

impl Plugin for SpellCastAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_spell_cast_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (spell_cast_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Clone)]
pub struct SpellCastAnimation {
    pub hit_send: bool,
    pub spell_cast: SpellCast,
}

fn init_spell_cast_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(spell_action) = action.downcast_ref::<SpellAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands
            .entity(entity)
            .insert(AnimationHolder(ActionAnimation::SpellCast(
                SpellCastAnimation {
                    hit_send: false,
                    spell_cast: spell_action.spell.cast.clone(),
                },
            )));
    }
}

fn spell_cast_animation(
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &Animator,
    )>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::SpellCast(animation)) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != animation.spell_cast.animation {
            animation_state.0 = animation.spell_cast.animation;
        }

        if animator.is_hit_frame() && !animation.hit_send {
            animation.hit_send = true;
            // TODO: otherwise we can push a component on the entity
            // Then simplify listen when this component is added to emit the next action
            // Remove this component when action animation is finished
            ev_animation_next.send(ActionAnimationNextEvent(entity));
        }

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));

            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}

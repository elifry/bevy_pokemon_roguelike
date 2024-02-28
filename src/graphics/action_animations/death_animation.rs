use bevy::{prelude::*, render::view::visibility};

use crate::actions::{death_action::DeathAction, RunningAction};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

const FLASH_NUMBER: u8 = 27;
const FLASH_DURATION_SECONDS: f32 = 0.02;

pub struct DeathAnimationPlugin;

impl Plugin for DeathAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_death_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (death_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Clone)]
pub struct DeathAnimation {
    pub attacker: Entity,
    pub flash_timer: Timer,
    pub flash_count: u8,
}

fn init_death_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (_entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(death_action) = action.downcast_ref::<DeathAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands
            .entity(death_action.target)
            .insert((AnimationHolder(ActionAnimation::Death(DeathAnimation {
                attacker: death_action.attacker,
                flash_timer: Timer::from_seconds(FLASH_DURATION_SECONDS, TimerMode::Once),
                flash_count: 0,
            })),));
    }
}

fn death_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationHolder, &mut Visibility)>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, mut visibility) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Death(death_animation)) = animation.as_mut() else {
            continue;
        };

        death_animation.flash_timer.tick(time.delta());

        if !death_animation.flash_timer.finished() {
            continue;
        }

        if death_animation.flash_count >= FLASH_NUMBER {
            ev_animation_finished.send(ActionAnimationFinishedEvent(death_animation.attacker));
            ev_animation_next.send(ActionAnimationNextEvent(death_animation.attacker));
            continue;
        }

        death_animation.flash_count += 1;

        if death_animation.flash_count == FLASH_NUMBER {
            death_animation.flash_timer = Timer::from_seconds(0.10, TimerMode::Once);
        } else {
            death_animation.flash_timer =
                Timer::from_seconds(FLASH_DURATION_SECONDS, TimerMode::Once);
        }

        *visibility = if death_animation.flash_count % 2 == 0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}

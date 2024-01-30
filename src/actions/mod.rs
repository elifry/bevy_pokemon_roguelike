use crate::{graphics::action_animation::AnimationPlayingEvent, pieces::Health, GamePlayingSet};
use bevy::prelude::*;
use dyn_clone::DynClone;
use std::{any::Any, collections::VecDeque, fmt::Debug};

pub mod damage_action;
pub mod melee_hit_action;
pub mod skip_action;
pub mod walk_action;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionExecutedEvent>()
            .add_event::<ActionProcessedEvent>()
            .add_event::<ProcessActionFailed>()
            .add_event::<ProcessingActionEvent>()
            .init_resource::<ActionQueue>()
            .add_systems(
                Update,
                (process_action_queue).in_set(GamePlayingSet::Actions),
            );
    }
}

dyn_clone::clone_trait_object!(Action);
pub trait Action: Send + Sync + DynClone + Debug {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
    fn is_parallel_execution(&self) -> bool;
}

// Execution Order of action
// ActionExecutedEvent -> ActionProcessedEvent / ProcessActionFailed

#[derive(Debug, Clone)]
pub struct QueuedAction {
    pub entity: Entity,
    pub performable_actions: Vec<Box<dyn Action>>,
}

#[derive(Resource, Default, Clone)]
pub struct ActionQueue(pub VecDeque<QueuedAction>);

/// Current executed action attached to an entity
#[derive(Component)]
pub struct RunningAction(pub Box<dyn Action>);

#[derive(Component, Default, Clone)]
pub struct NextActions(pub Vec<Box<dyn Action>>);

#[derive(Event, Debug)]
pub struct ProcessingActionEvent;

#[derive(Event, Debug)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);

#[derive(Event, Debug)]
pub struct ActionProcessedEvent;

#[derive(Event, Debug)]
pub struct ProcessActionFailed;

pub fn process_action_queue(world: &mut World) {
    let mut ev_animation_playing = world
        .get_resource_mut::<Events<AnimationPlayingEvent>>()
        .unwrap();

    if ev_animation_playing
        .get_reader()
        .read(&ev_animation_playing)
        .len()
        > 0
    {
        // Weird issue, the events doesn't get clear without thats
        ev_animation_playing.clear();
        world.send_event(ProcessingActionEvent);
        return;
    }

    'queue_action_loop: loop {
        // Get the first action of the queue
        let queued_action = {
            let action_queue = world.get_resource_mut::<ActionQueue>();

            if let Some(mut queue) = action_queue {
                if queue.0.is_empty() {
                    // If the ActionQueue is empty, return
                    break;
                }
                queue.0.pop_front()
            } else {
                // If there's no ActionQueue, return
                break;
            }
        };

        let Some(queued_action) = queued_action else {
            // If there is no more action in the queue
            break;
        };

        world.send_event(ProcessingActionEvent);

        if let Ok(health) = world.query::<&Health>().get(world, queued_action.entity) {
            if health.value == 0 {
                info!("{:?} is dead ", queued_action.entity);
                break;
            }
        }

        for (action_index, action) in queued_action.performable_actions.iter().enumerate() {
            match action.execute(world) {
                Ok(result_actions) => {
                    // Action well executed (insert the `RunningAction`)
                    info!("action executed {:?}", action);
                    world
                        .entity_mut(queued_action.entity)
                        .insert(RunningAction(action.clone()));

                    if !result_actions.is_empty() {
                        let mut action_queue = world.get_resource_mut::<ActionQueue>().unwrap();
                        action_queue.0.push_front(QueuedAction {
                            entity: queued_action.entity,
                            performable_actions: result_actions,
                        });
                    }

                    if action.is_parallel_execution() {
                        // go to the next queue action
                        break;
                    }

                    // Avoid processing multiple action at the same time
                    break 'queue_action_loop;
                }
                Err(_) => {
                    warn!("Action not valid");
                    if action_index == queued_action.performable_actions.len() - 1 {
                        // Last performable action is also invalid
                        warn!("No more performable action");
                        break;
                    }
                }
            };
        }

        apply_deferred(world);
    }
}

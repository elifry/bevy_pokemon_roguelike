use crate::{GamePlayingSet, GameState};
use bevy::prelude::*;
use dyn_clone::DynClone;
use std::{any::Any, collections::VecDeque, fmt::Debug};

use self::walk_action::WalkAction;

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
            .init_resource::<ActionQueue>()
            .add_systems(
                Update,
                (process_action_queue, apply_deferred).in_set(GamePlayingSet::Action),
            );
    }
}

dyn_clone::clone_trait_object!(Action);
pub trait Action: Send + Sync + DynClone + Debug {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
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

#[derive(Event)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);

#[derive(Event)]
pub struct ActionProcessedEvent;

#[derive(Event)]
pub struct ProcessActionFailed;

fn process_action_queue(world: &mut World) {
    // Early return if there are running actions
    if world
        .query_filtered::<Entity, With<RunningAction>>()
        .get_single(world)
        .is_ok()
    {
        return;
    }

    'queue_action_loop: loop {
        // Get the first action of the queue
        let queued_action = {
            let action_queue = world.get_resource_mut::<ActionQueue>();
            if let Some(mut queue) = action_queue {
                if queue.0.is_empty() {
                    // If the ActionQueue is empty, break the loop
                    break;
                }
                queue.0.pop_front()
            } else {
                // If there's no ActionQueue, break the loop
                break;
            }
        };

        let Some(queued_action) = queued_action else {
            // If there is no more action in the queue
            break;
        };

        for (action_index, action) in queued_action.performable_actions.iter().enumerate() {
            match action.execute(world) {
                Ok(result_actions) => {
                    // Action well executed (insert the `RunningAction`)
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

                    let parallel_execution = action.as_any().downcast_ref::<WalkAction>().is_some();

                    if !parallel_execution {
                        // If no parallel execution, break out of the outer loop
                        break 'queue_action_loop;
                    }

                    break;
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
    }
}

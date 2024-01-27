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
            .add_systems(Update, process_action_queue.in_set(GamePlayingSet::Action));
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
    let mut query_running_actions = world.query_filtered::<Entity, With<RunningAction>>();
    if query_running_actions.get_single(world).is_ok() {
        // there is running actions, we do not want to process futhermore
        return;
    }

    let cloned_action_queue = world.get_resource::<ActionQueue>().unwrap().clone();

    if cloned_action_queue.0.is_empty() {
        return;
    }

    // we clone the action_queue because we're gonna modify the original action_queue
    'queue_action_loop: for queued_action in cloned_action_queue.0.iter() {
        for (action_index, action) in queued_action.performable_actions.iter().enumerate() {
            // TODO: add the result actions to the queue
            let Ok(_result_actions) = action.execute(world) else {
                // not valid action
                warn!("Action not valid");
                if action_index == queued_action.performable_actions.len() - 1 {
                    // last performable action is also invalid
                    // we remove it from the list
                    world
                        .get_resource_mut::<ActionQueue>()
                        .unwrap()
                        .0
                        .pop_front();
                }

                continue;
            };

            let parallel_execution = action.as_any().downcast_ref::<WalkAction>().is_some();

            world
                .entity_mut(queued_action.entity)
                .insert(RunningAction(action.clone()));

            // pop the executed action
            world
                .get_resource_mut::<ActionQueue>()
                .unwrap()
                .0
                .pop_front();

            if parallel_execution {
                break;
            }
            break 'queue_action_loop;
        }
    }
}

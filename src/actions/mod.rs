use bevy::prelude::*;
use std::{any::Any, collections::VecDeque};

use crate::{
    pieces::{ActiveActor, Actor},
    player::Player,
    GameState,
};

pub mod walk_action;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<ActionExecutedEvent>()
            .add_event::<ActionProcessedEvent>()
            .add_event::<ProcessActionFailed>()
            .add_systems(
                Update,
                process_action_queue
                    .run_if(on_event::<TickEvent>())
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct ActionProcessedEvent;

#[derive(Event)]
pub struct ProcessActionFailed;

fn process_action_queue(world: &mut World) {
    let mut active_actor_query = world.query_filtered::<(Entity, &mut Actor), With<ActiveActor>>();

    let Ok((entity, mut actor)) = active_actor_query.get_single_mut(world) else {
        // this can mean that the current actor
        // has been removed from the world since creating the queue
        // cue the next one
        // world.send_event(NextActorEvent);
        return;
    };

    info!("process_action_queue for {:?}", entity);

    let possible_actions = actor.0.drain(..).collect::<Vec<_>>();

    let mut success = false;
    for action in possible_actions {
        success = success || execute_action(action, world);
        if success {
            break;
        }
    }

    if !success && world.get::<Player>(entity).is_some() {
        info!("Invalid player action");
        world.send_event(ProcessActionFailed);
        return;
    }

    world.send_event(ActionProcessedEvent);
}

fn execute_action(action: Box<dyn Action>, world: &mut World) -> bool {
    if let Ok(next_actions) = action.execute(world) {
        if let Some(mut pending_actions) = world.get_resource_mut::<PendingActions>() {
            pending_actions.0.extend(next_actions);
        }
        info!("action executed");
        world.send_event(ActionExecutedEvent(action));
        return true;
    }
    return false;
}

use bevy::prelude::*;
use std::any::Any;

use crate::{
    pieces::Actor,
    player::Player,
    turn::{CurrentActor, TurnState},
    GameState,
};

pub mod damage_action;
pub mod melee_hit_action;
pub mod walk_action;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
            .init_resource::<PendingActions>()
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

// Execution Order of action
// ActionExecutedEvent -> ActionProcessedEvent -> ActionFinishedEvent

#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

#[derive(Event)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct ActionProcessedEvent;

#[derive(Event)]
pub struct ProcessActionFailed;

fn process_action_queue(world: &mut World) {
    if process_pending_actions(world) {
        return;
    }

    let current_actor = world.get_resource::<CurrentActor>().unwrap();

    let Some(entity) = current_actor.0 else {
        return;
    };

    let mut actor_query = world.query::<&mut Actor>();

    let Ok(mut actor) = actor_query.get_mut(world, entity) else {
        // otherwise the actor has been despawned
        world.send_event(ActionProcessedEvent);
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

fn process_pending_actions(world: &mut World) -> bool {
    // returns true if at least one pending action has been processed
    // take action objects without holding the mutable reference to the world
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        _ => return false,
    };
    let mut success = false;
    for action in pending {
        success = success || execute_action(action, world);
    }
    success
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
    false
}

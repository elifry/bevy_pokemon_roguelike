use std::collections::VecDeque;

use bevy::{prelude::*};

use crate::{
    actions::{PendingActions, TickEvent},
    graphics::GraphicsWaitEvent,
    pieces::{Actor, PieceDeathEvent},
    player::{Player},
    GameState,
};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentActor>()
            .init_resource::<ActorQueue>()
            .add_event::<NextActorEvent>()
            .add_state::<TurnState>()
            .add_systems(OnEnter(TurnState::TurnTook), turn_took)
            .add_systems(OnEnter(GameState::Playing), handle_game_start)
            .add_systems(
                Update,
                check_end_turn.run_if(in_state(TurnState::TakingTurn)),
            )
            .add_systems(
                Update,
                add_actor_to_queue.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, taking_turn.run_if(on_event::<TickEvent>()))
            .add_systems(Update, handle_actor_death);
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum TurnState {
    #[default]
    None,
    PlayerTurn,
    NPCTurn,
    TakingTurn,
    TurnTook,
}

#[derive(Resource, Default)]
pub struct CurrentActor(pub Option<Entity>);

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct NextActorEvent;

fn taking_turn(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::TakingTurn);
}

fn check_end_turn(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>,
    pending_actions: Res<PendingActions>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if ev_wait.read().len() > 0 {
        return;
    }

    if !pending_actions.0.is_empty() {
        ev_tick.send(TickEvent);
        return;
    }

    next_state.set(TurnState::TurnTook);
}

fn turn_took(
    mut ev_next_actor: EventWriter<NextActorEvent>,
    player_query: Query<&Player>,
    actor_queue: Res<ActorQueue>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut res_current_actor: ResMut<CurrentActor>,
) {
    info!("Turn took");

    let Some(current_actor) = res_current_actor.0 else {
        return;
    };

    let current_actor_index = actor_queue
        .0
        .iter()
        .position(|actor| *actor == current_actor)
        .unwrap();

    let next_actor_index = (current_actor_index + 1) % actor_queue.0.len();
    let next_actor = actor_queue.0[next_actor_index];

    res_current_actor.0 = Some(next_actor);

    let is_player_turn = player_query.get(next_actor).is_ok();

    if is_player_turn {
        next_state.set(TurnState::PlayerTurn);
    } else {
        next_state.set(TurnState::NPCTurn);
    }

    ev_next_actor.send(NextActorEvent);
}

fn handle_actor_death(
    mut actor_queue: ResMut<ActorQueue>,
    mut ev_piece_death: EventReader<PieceDeathEvent>,
) {
    for ev in ev_piece_death.read() {
        let death_actor_index = actor_queue
            .0
            .iter()
            .position(|entity| *entity == ev.entity)
            .unwrap();

        info!("Removed {:?} from the actor queue", ev.entity);

        actor_queue.0.remove(death_actor_index);

        // let Some(current_actor) = res_current_actor.0 else {
        //     return;
        // };

        // let is_next_actor = next_actor_query.get(ev.entity).is_ok();

        // info!("is next actor {}", is_next_actor);

        // if current_actor != ev.entity && !is_next_actor {
        //     info!("not next actor or current actor");
        //     return;
        // }

        // info!("emit next actor");

        // let next_actor_index = (death_actor_index + 1) % actor_queue.0.len();
        // let next_actor = actor_queue.0[next_actor_index];

        // commands.entity(next_actor).insert(NextActor);
    }
}

fn handle_game_start(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::PlayerTurn);
}

fn add_actor_to_queue(
    query: Query<Entity, Added<Actor>>,
    player_query: Query<&Player>,
    mut actor_queue: ResMut<ActorQueue>,
    mut current_actor: ResMut<CurrentActor>,
) {
    for entity in query.iter() {
        info!("Add {:?} to actor queue", entity);
        actor_queue.0.push_back(entity);

        if let Ok(_player) = player_query.get(entity) {
            current_actor.0 = Some(entity);
        }
    }
}

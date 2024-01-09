use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::ActionFinishedEvent,
    pieces::{Actor, PieceDeathEvent},
    player::{Player, PlayerActionEvent},
    GameState,
};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentActor>()
            .init_resource::<ActorQueue>()
            .add_event::<NextActorEvent>()
            .add_state::<TurnState>()
            .add_systems(
                Update,
                handle_action_finished.run_if(on_event::<ActionFinishedEvent>()),
            )
            .add_systems(OnEnter(GameState::Playing), handle_game_start)
            .add_systems(
                Update,
                player_taking_action.run_if(on_event::<PlayerActionEvent>()),
            )
            .add_systems(
                Update,
                add_actor_to_queue.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, handle_actor_death);
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum TurnState {
    #[default]
    None,
    PlayerTurn,
    TakingTurn,
    NPCTurn,
}

#[derive(Resource, Default)]
pub struct CurrentActor(pub Option<Entity>);

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct NextActorEvent;

fn handle_actor_death(
    mut actor_queue: ResMut<ActorQueue>,
    mut ev_piece_death: EventReader<PieceDeathEvent>,
) {
    for ev in ev_piece_death.read() {
        let actor_index = actor_queue
            .0
            .iter()
            .position(|entity| *entity == ev.entity)
            .unwrap();
        info!("Removed {:?} from the actor queue", ev.entity);
        actor_queue.0.remove(actor_index);

        // TODO: set up next actor if it was the current actor
    }
}

fn player_taking_action(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::TakingTurn);
}

fn handle_game_start(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::PlayerTurn);
}

fn handle_action_finished(
    actor_queue: Res<ActorQueue>,
    player_query: Query<&Player>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut res_current_actor: ResMut<CurrentActor>,
    mut ev_next_actor: EventWriter<NextActorEvent>,
) {
    let Some(current_actor) = res_current_actor.0 else {
        return;
    };
    let current_actor_index = actor_queue
        .0
        .iter()
        .position(|actor| *actor == current_actor)
        .unwrap();

    let next_actor_index = (current_actor_index + 1) % actor_queue.0.len();

    let next_actor = actor_queue.0.get(next_actor_index).copied();

    info!("-- Next actor turn {:?}", next_actor);

    if let Ok(_player) = player_query.get(next_actor.unwrap()) {
        next_state.set(TurnState::PlayerTurn);
    } else {
        next_state.set(TurnState::NPCTurn);
    }

    res_current_actor.0 = next_actor;

    ev_next_actor.send(NextActorEvent);
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

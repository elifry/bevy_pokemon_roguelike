use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::systems::process_input_system;

use crate::{
    actions::{
        process_action_queue, ActionQueue, NextActions, ProcessingActionEvent, QueuedAction,
        RunningAction,
    },
    pieces::{Actor, PieceDeathEvent},
    player::{Player, PlayerActionEvent},
    GamePlayingSet, GameState, TurnState,
};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnOrder>()
            .add_systems(
                Update,
                (add_actor_to_queue, turn_system)
                    .chain()
                    .in_set(GamePlayingSet::TurnLogics),
            )
            .add_systems(
                Update,
                handle_player_action_event.run_if(on_event::<PlayerActionEvent>()),
            )
            .add_systems(
                Update,
                (check_player_turn)
                    .chain()
                    .in_set(GamePlayingSet::LateLogics), //.run_if(in_state(TurnState::ProcessAction)),
            )
            .add_systems(Update, handle_actor_death);
    }
}

#[derive(Default, Resource)]
pub struct TurnOrder(pub VecDeque<Entity>);

fn handle_player_action_event(mut next_state: ResMut<NextState<TurnState>>) {
    info!("Player took action");
    next_state.set(TurnState::Logics);
}

fn check_player_turn(
    mut ev_wait: EventReader<ProcessingActionEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if ev_wait.read().len() == 0 {
        info!("turn input state");
        next_state.set(TurnState::Input);
    }
}

pub fn turn_system(
    turn_order: ResMut<TurnOrder>,
    query_player: Query<Entity, With<Player>>,
    query_next_actions: Query<&NextActions>,
    query_running_actions: Query<&RunningAction>,
    mut action_queue: ResMut<ActionQueue>,
    mut event_player_action: EventReader<PlayerActionEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if query_running_actions.get_single().is_ok() {
        return;
    }

    let Some(player_action) = event_player_action.read().next() else {
        return;
    };
    info!("Execute turn order");

    action_queue.0.clear();
    for actor_turn in turn_order.0.iter() {
        let is_player = query_player.get(*actor_turn).is_ok();

        if is_player {
            let actions = player_action.0.clone();

            action_queue.0.push_back(QueuedAction {
                entity: *actor_turn,
                performable_actions: actions,
            });
            continue;
        }

        let Ok(next_actions) = query_next_actions.get(*actor_turn) else {
            warn!(
                "{:?} do not have a next action component during its turn",
                *actor_turn
            );
            continue;
        };
        let actions = next_actions.0.clone();
        action_queue.0.push_back(QueuedAction {
            entity: *actor_turn,
            performable_actions: actions,
        });
    }

    event_player_action.clear();

    next_state.set(TurnState::ProcessAction);
}

fn handle_actor_death(
    mut actor_queue: ResMut<TurnOrder>,
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

fn add_actor_to_queue(
    query: Query<Entity, Added<Actor>>,
    player_query: Query<&Player>,
    mut turn_order: ResMut<TurnOrder>,
) {
    for entity in query.iter() {
        info!("Add {:?} to turn order", entity);
        turn_order.0.push_back(entity);
    }
}

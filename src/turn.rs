use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::{ActionQueue, NextActions, ProcessingActionEvent, QueuedAction},
    graphics::action_animation::{AnimationHolder},
    pieces::{Actor, Health},
    player::{Player, PlayerActionEvent},
    GamePlayingSet,
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
                handle_actor_death.in_set(GamePlayingSet::LateLogics),
            );
    }
}

#[derive(Default, Resource)]
pub struct TurnOrder(pub VecDeque<Entity>);

pub fn turn_system(
    turn_order: ResMut<TurnOrder>,
    query_player: Query<Entity, With<Player>>,
    query_next_actions: Query<&NextActions>,
    mut action_queue: ResMut<ActionQueue>,
    mut event_player_action: EventReader<PlayerActionEvent>,
) {
    let Some(player_action) = event_player_action.read().next() else {
        return;
    };

    info!("--------------- Turn ---------------");
    info!("------------------------------------");

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
}

fn handle_actor_death(
    mut actor_queue: ResMut<TurnOrder>,
    query_health: Query<(Entity, &Health), Without<AnimationHolder>>,
    mut commands: Commands,
    mut ev_processing_action: EventReader<ProcessingActionEvent>,
) {
    if ev_processing_action.read().len() > 0 {
        return;
    }

    for (entity, health) in query_health.iter() {
        if !health.is_dead() {
            continue;
        }
        let death_actor_index = actor_queue.0.iter().position(|e| *e == entity).unwrap();

        info!("Removed {:?} from the actor queue", entity);

        actor_queue.0.remove(death_actor_index);

        commands.entity(entity).despawn();
    }
}

fn add_actor_to_queue(query: Query<Entity, Added<Actor>>, mut turn_order: ResMut<TurnOrder>) {
    for entity in query.iter() {
        info!("Add {:?} to turn order", entity);
        turn_order.0.push_back(entity);
    }
}

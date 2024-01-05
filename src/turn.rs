use bevy::prelude::*;

use crate::{
    actions::{ActionProcessedEvent, ActorQueue, TickEvent},
    pieces::{ActiveActor, Actor, Piece, PieceKind},
    GameState,
};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_action_processed_event.run_if(on_event::<ActionProcessedEvent>()),
        )
        .add_systems(
            Update,
            add_actor_to_queue.run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_action_processed_event(
    mut commands: Commands,
    actor_queue: ResMut<ActorQueue>,
    mut active_actor_query: Query<Entity, With<ActiveActor>>,
) {
    let Ok(entity) = active_actor_query.get_single() else {
        return;
    };
    info!("End turn");
    // commands.entity(entity).remove::<ActiveActor>();

    // TODO:
    // remove current active actor
    // select the next current active actor
    // set the active actor
}

fn add_actor_to_queue(
    mut commands: Commands,
    query: Query<(Entity, &Piece), Added<Actor>>,
    mut actor_queue: ResMut<ActorQueue>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    for (entity, piece) in query.iter() {
        info!("Add {:?} to actor queue", entity);
        actor_queue.0.push_back(entity);

        if let PieceKind::Player = piece.kind {
            commands.entity(entity).insert(ActiveActor);
            ev_tick.send(TickEvent)
        }
    }
}

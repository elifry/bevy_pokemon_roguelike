use bevy::prelude::*;

use crate::actions::walk_action::WalkAction;
use crate::actions::{Action, TickEvent};
use crate::game_control::{GameControl, GameControlEvent};
use crate::map::Position;
use crate::pieces::{Actor, Health, Occupier, Piece, PieceKind};
use crate::turn::{CurrentActor, TurnState};
use crate::vector2_int::Vector2Int;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, take_action.run_if(in_state(TurnState::PlayerTurn)));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerActionEvent;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Occupier,
        Health { value: 10 },
        Actor::default(),
        Piece {
            kind: PieceKind::Player,
        },
        Position(Vector2Int::new(0, 0)),
    ));
}

fn take_action(
    mut ev_game_control: EventReader<GameControlEvent>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
    current_actor: Res<CurrentActor>,
    mut ev_tick: EventWriter<TickEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,
) {
    for ev in ev_game_control.read() {
        let Ok((entity, mut actor)) = player_query.get_single_mut() else {
            return;
        };

        let Some(current_actor) = current_actor.0 else {
            return;
        };

        if current_actor != entity {
            return;
        }

        let GameControlEvent(GameControl::Target(target)) = ev else {
            continue;
        };

        let walk_action = Box::new(WalkAction {
            entity,
            targeted_position: *target,
        }) as Box<dyn Action>;

        actor.0 = vec![walk_action];

        ev_tick.send(TickEvent);

        ev_action.send(PlayerActionEvent);
    }
}

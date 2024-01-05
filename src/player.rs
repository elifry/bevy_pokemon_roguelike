use bevy::prelude::*;

use crate::actions::walk_action::WalkAction;
use crate::actions::{Action, TickEvent};
use crate::game_control::{GameControl, GameControlEvent};
use crate::map::Position;
use crate::pieces::{Actor, Health, Occupier, Piece, PieceKind};
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, take_action.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

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
        Position(IVec2::new(0, 0)),
    ));
}

fn take_action(
    mut ev_game_control: EventReader<GameControlEvent>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    for ev in ev_game_control.read() {
        let GameControlEvent(GameControl::Target(target)) = ev else {
            continue;
        };

        let Ok((entity, mut actor)) = player_query.get_single_mut() else {
            return;
        };

        let walk_action = Box::new(WalkAction {
            entity,
            targeted_position: *target,
        }) as Box<dyn Action>;

        actor.0 = vec![walk_action];

        ev_tick.send(TickEvent);
    }
}

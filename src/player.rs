use bevy::prelude::*;

use crate::actions::melee_hit_action::MeleeHitAction;
use crate::actions::skip_action::SkipAction;
use crate::actions::walk_action::WalkAction;
use crate::actions::{Action, TickEvent};
use crate::game_control::{GameControl, GameControlEvent};
use crate::map::Position;
use crate::pieces::{Actor, Health, Occupier, Orientation, Piece, PieceKind};
use crate::pokemons::{Pokemon, Pokemons};
use crate::turn::{CurrentActor, TurnState};
use crate::vector2_int::Vector2Int;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_systems(OnEnter(GameState::Initializing), spawn_player)
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
        Pokemon(Pokemons::Charmander),
        Player,
        Occupier,
        Health { value: 10 },
        Actor::default(),
        Piece {
            kind: PieceKind::Player,
        },
        Position(Vector2Int::new(1, 0)),
    ));
}

fn take_action(
    mut ev_game_control: EventReader<GameControlEvent>,
    mut player_query: Query<(Entity, &mut Actor, &Position), With<Player>>,
    target_query: Query<(Entity, &Position), With<Health>>,
    current_actor: Res<CurrentActor>,
    mut ev_tick: EventWriter<TickEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,
) {
    for ev in ev_game_control.read() {
        let Ok((entity, mut actor, position)) = player_query.get_single_mut() else {
            return;
        };

        let Some(current_actor) = current_actor.0 else {
            return;
        };

        if current_actor != entity {
            return;
        }

        let action = match ev.0 {
            GameControl::Target(target) => {
                // check if there a target when the player move
                let target_entities = target_query
                    .iter()
                    .filter(|(_, p)| p.0 == target)
                    .collect::<Vec<_>>();

                let action: Box<dyn Action> = if !target_entities.is_empty() {
                    Box::new(MeleeHitAction {
                        attacker: entity,
                        target,
                        damage: 1,
                    })
                } else {
                    Box::new(WalkAction {
                        entity,
                        from: position.0,
                        to: target,
                    })
                };

                action
            }
            GameControl::Skip => Box::new(SkipAction),
        };

        actor.0 = vec![action];

        ev_tick.send(TickEvent);

        ev_action.send(PlayerActionEvent);
    }
}

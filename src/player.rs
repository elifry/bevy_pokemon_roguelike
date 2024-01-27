use bevy::prelude::*;

use crate::actions::melee_hit_action::MeleeHitAction;
use crate::actions::skip_action::SkipAction;
use crate::actions::walk_action::WalkAction;
use crate::actions::{Action, RunningAction};
use crate::game_control::{GameControl, GameControlEvent};
use crate::map::Position;
use crate::pieces::{Actor, FacingOrientation, Health, Occupier, Orientation, Piece, PieceKind};
use crate::pokemons::{Pokemon, Pokemons};
use crate::vector2_int::Vector2Int;
use crate::{GamePlayingSet, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_systems(OnEnter(GameState::Initializing), spawn_player)
            .add_systems(Update, take_action.after(GamePlayingSet::Input));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Event, Debug, Default)]
pub struct PlayerActionEvent(pub Vec<Box<dyn Action>>);

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        FacingOrientation(Orientation::South),
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
    mut player_query: Query<
        (Entity, &mut Actor, &Position),
        (With<Player>, Without<RunningAction>),
    >,
    target_query: Query<(Entity, &Position), With<Health>>,
    mut ev_action: EventWriter<PlayerActionEvent>,
) {
    let Ok((entity, mut actor, position)) = player_query.get_single_mut() else {
        ev_game_control.clear();
        return;
    };

    for ev in ev_game_control.read() {
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

        ev_action.send(PlayerActionEvent(vec![action]));
    }
}

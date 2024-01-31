use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

use crate::actions::destroy_wall_action::DestroyWallAction;
use crate::actions::melee_hit_action::MeleeHitAction;
use crate::actions::skip_action::SkipAction;
use crate::actions::walk_action::WalkAction;
use crate::actions::{Action, ActionQueueProcessedEvent};
use crate::map::Position;
use crate::pieces::{Actor, FacingOrientation, Health, Occupier, Orientation, Piece, PieceKind};
use crate::pokemons::{Pokemon, PokemonID};
use crate::vector2_int::Vector2Int;
use crate::{GamePlayingSet, GameState};

pub struct PlayerPlugin;

const DIR_KEY_MAPPING: [(PlayerAction, Vector2Int); 4] = [
    (PlayerAction::Up, Vector2Int { x: 0, y: 1 }),
    (PlayerAction::Down, Vector2Int { x: 0, y: -1 }),
    (PlayerAction::Left, Vector2Int { x: -1, y: 0 }),
    (PlayerAction::Right, Vector2Int { x: 1, y: 0 }),
];

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::Initializing), (spawn_player))
            .add_systems(Update, take_action.in_set(GamePlayingSet::Controls));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Event, Debug, Default)]
pub struct PlayerActionEvent(pub Vec<Box<dyn Action>>);

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
    Skip,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        FacingOrientation(Orientation::South),
        Pokemon(PokemonID::Charmander),
        Player,
        Occupier,
        Health { value: 10 },
        Actor::default(),
        Piece {
            kind: PieceKind::Player,
        },
        Position(Vector2Int::new(4, 4)),
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Space, PlayerAction::Skip),
                (KeyCode::W, PlayerAction::Up),
                (KeyCode::Up, PlayerAction::Up),
                (KeyCode::S, PlayerAction::Down),
                (KeyCode::Down, PlayerAction::Down),
                (KeyCode::A, PlayerAction::Left),
                (KeyCode::Left, PlayerAction::Left),
                (KeyCode::D, PlayerAction::Right),
                (KeyCode::Right, PlayerAction::Right),
            ]),
        },
    ));
}

fn take_action(
    player_query: Query<(Entity, &ActionState<PlayerAction>, &Position), With<Player>>,
    mut ev_action_queue_processed: EventReader<ActionQueueProcessedEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,
    mut is_taking_action: Local<bool>,
) {
    if ev_action_queue_processed.read().len() > 0 {
        info!("Player can take action");
        *is_taking_action = false;
        ev_action_queue_processed.clear();
    }

    if *is_taking_action {
        return;
    }

    let Ok((entity, action_state, position)) = player_query.get_single() else {
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if !action_state.pressed(key) {
            continue;
        }
        let target = position.0 + dir;

        let walk_action = Box::new(WalkAction {
            entity,
            from: position.0,
            to: target,
        }) as Box<dyn Action>;

        let attack_action = Box::new(MeleeHitAction {
            attacker: entity,
            damage: 1,
            target,
        }) as Box<dyn Action>;

        let destroy_wall = Box::new(DestroyWallAction {
            instigator: entity,
            target,
        }) as Box<dyn Action>;

        info!("Send player action event");
        *is_taking_action = true;
        ev_action.send(PlayerActionEvent(vec![
            walk_action,
            attack_action,
            destroy_wall,
        ]));
        return;
    }

    if action_state.pressed(PlayerAction::Skip) {
        let action = Box::new(SkipAction);
        *is_taking_action = true;
        ev_action.send(PlayerActionEvent(vec![action]));
    }
}

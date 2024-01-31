mod npc;

use bevy::{ecs::entity, prelude::*};
use rand::{thread_rng, Rng};

use crate::{
    actions::{walk_action::WalkAction, Action, NextActions},
    map::{GameMap, Position},
    pieces::{Actor, Occupier},
    player::Player,
    vector2_int::{utils::find_path, Vector2Int, ORTHO_DIRECTIONS},
    GamePlayingSet, GameState,
};

use self::npc::NPCBundle;

const PLAYER_ATTACK_SCORE: i32 = 100;
const MOVE_SCORE: i32 = 50;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (AISet::Planning, AISet::Late)
                .chain()
                .in_set(GamePlayingSet::AI),
        )
        .add_systems(Update, plan_walk.in_set(AISet::Planning))
        .add_systems(
            Update,
            (npc_action, apply_deferred).chain().in_set(AISet::Late),
        )
        .add_systems(OnEnter(GameState::Playing), spawn_npcs);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum AISet {
    Planning, // AI Planning
    Late,     // AI taking their actions
}

#[derive(Component)]
struct AI;

#[derive(Component, Default, Clone)]
struct PossibleActions(Vec<PossibleAction>);

#[derive(Clone, Debug)]
struct PossibleAction {
    score: i32,
    action: Box<dyn Action>,
}

fn spawn_npcs(mut commands: Commands) {
    spawn_test_npc(&mut commands, Vector2Int::new(5, 5));
    spawn_test_npc(&mut commands, Vector2Int::new(3, 5));
}

fn spawn_test_npc(commands: &mut Commands, position: Vector2Int) {
    commands.spawn(NPCBundle::new("NPC".to_string(), position));
}

fn npc_action(mut query: Query<(Entity, &mut PossibleActions), With<AI>>, mut commands: Commands) {
    for (entity, mut possible_actions) in query.iter_mut() {
        let mut possible_actions = possible_actions.0.drain(..).collect::<Vec<_>>();

        possible_actions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let possible_actions = possible_actions
            .drain(..)
            .map(|possible_action| possible_action.action)
            .collect::<Vec<_>>();

        commands
            .entity(entity)
            .insert(NextActions(possible_actions));
    }
}

fn plan_walk(
    mut query: Query<(Entity, &Position, &mut PossibleActions), With<AI>>,
    player_query: Query<&Position, With<Player>>,
    occupier_query: Query<&Position, With<Occupier>>,
    map: Res<GameMap>,
) {
    let Ok(player_position) = player_query.get_single() else {
        return;
    };

    for (entity, position, mut possible_actions) in query.iter_mut() {
        // get all possible move targets
        let positions = ORTHO_DIRECTIONS
            .iter()
            .map(|d| *d + position.0)
            .collect::<Vec<_>>();

        // find possible path to the player
        let path_to_player = find_path(
            position.0,
            player_position.0,
            &map.tiles.keys().cloned().collect(),
            &occupier_query.iter().map(|p| p.0).collect(),
        );
        let mut rng = thread_rng();
        let walk_possible_actions = positions
            .iter()
            .map(|v| {
                // randomize movement choices
                let mut d = rng.gen_range(-10..0);
                if let Some(path) = &path_to_player {
                    // however prioritze a movement if it leads to the player
                    if path.contains(v) {
                        d = 15
                    }
                }
                PossibleAction {
                    action: Box::new(WalkAction {
                        entity,
                        from: position.0,
                        to: *v,
                    }) as Box<dyn Action>,
                    score: MOVE_SCORE + d,
                }
            })
            .collect::<Vec<_>>();

        possible_actions.0.extend(walk_possible_actions);
    }
}

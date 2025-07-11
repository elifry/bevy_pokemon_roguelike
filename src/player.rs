use bevy::prelude::*;
use char_animation::anim_key::AnimKey;
use char_animation::orientation::Orientation;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use rand::Rng;

use crate::actions::destroy_wall_action::DestroyWallAction;
use crate::actions::melee_hit_action::MeleeHitAction;
use crate::actions::skip_action::SkipAction;
use crate::actions::spell_action::SpellAction;
use crate::actions::walk_action::WalkAction;
use crate::actions::{Action, ProcessingActionEvent};
use crate::faction::Faction;
use crate::map::Position;
use crate::move_type::MoveCategory;
use crate::pieces::{Actor, FacingOrientation, Occupier, Piece, PieceKind};
use crate::pokemons::Pokemon;
use crate::spells::{ProjectileSpell, Spell, SpellCast, SpellHit, SpellType};
use crate::{GamePlayingSet, GameState};

pub struct PlayerPlugin;

const DIR_KEY_MAPPING: [(PlayerAction, IVec2); 4] = [
    (PlayerAction::Up, IVec2 { x: 0, y: 1 }),
    (PlayerAction::Down, IVec2 { x: 0, y: -1 }),
    (PlayerAction::Left, IVec2 { x: -1, y: 0 }),
    (PlayerAction::Right, IVec2 { x: 1, y: 0 }),
];

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::Initializing), spawn_player)
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
    SpellSlot1,
    SpellSlot2,
    SpellSlot3,
    SpellSlot4,
}

fn spawn_player(
    mut commands: Commands,
    pokemon_char_assets: Res<crate::graphics::assets::pokemon_chara_assets::PokemonCharaAssets>,
) {
    // TODO: This random Pokemon selection is temporary for testing purposes.
    // In the future, this will be replaced with player choice during game start/character creation.
    let available_pokemon_ids: Vec<u32> = pokemon_char_assets.0.keys().cloned().collect();

    if available_pokemon_ids.is_empty() {
        panic!("No Pokemon character assets loaded! Check the assets/chara directory.");
    }

    let starter_id =
        available_pokemon_ids[rand::thread_rng().gen_range(0..available_pokemon_ids.len())];

    info!("Starting with Pokemon ID: {}", starter_id);

    commands.spawn((
        Name::new("Player"),
        FacingOrientation(Orientation::South),
        Pokemon {
            id: starter_id,
            form_index: 0,
        },
        Faction::Player,
        Player,
        Occupier,
        Actor,
        Piece {
            kind: PieceKind::Player,
        },
        Position(IVec2::new(4, 4)),
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (PlayerAction::Skip, KeyCode::Space),
                (PlayerAction::Skip, KeyCode::Space),
                (PlayerAction::Up, KeyCode::KeyW),
                (PlayerAction::Up, KeyCode::ArrowUp),
                (PlayerAction::Down, KeyCode::KeyS),
                (PlayerAction::Down, KeyCode::ArrowDown),
                (PlayerAction::Left, KeyCode::KeyA),
                (PlayerAction::Left, KeyCode::ArrowLeft),
                (PlayerAction::Right, KeyCode::KeyD),
                (PlayerAction::Right, KeyCode::ArrowRight),
                (PlayerAction::SpellSlot1, KeyCode::Digit1),
                (PlayerAction::SpellSlot2, KeyCode::Digit2),
                (PlayerAction::SpellSlot3, KeyCode::Digit3),
                (PlayerAction::SpellSlot4, KeyCode::Digit4),
            ]),
        },
    ));
}

fn take_action(
    player_query: Query<(Entity, &ActionState<PlayerAction>, &Position), With<Player>>,
    mut ev_processing_action: EventReader<ProcessingActionEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,
) {
    if ev_processing_action.read().len() > 0 {
        // info!("Player can take action");
        ev_processing_action.clear();
        return;
    }

    let Ok((entity, action_state, position)) = player_query.get_single() else {
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if !action_state.pressed(&key) {
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
        ev_action.send(PlayerActionEvent(vec![
            walk_action,
            attack_action,
            destroy_wall,
        ]));
        return;
    }

    if action_state.pressed(&PlayerAction::SpellSlot1) {
        let action = Box::new(SpellAction {
            caster: entity,
            spell: Spell {
                name: "Flamethrower",
                range: 1..=3,
                spell_type: SpellType::Projectile(ProjectileSpell {
                    visual_effect: "Flamethrower_2",
                }),
                hit: SpellHit {
                    visual_effect: "Flamethrower",
                    damage: 1,
                    move_type: MoveCategory::Special,
                },
                cast: SpellCast {
                    visual_effect: "Circle_Small_Blue_In",
                    animation: AnimKey::Shoot,
                }, // Damage visual effect: Hit_Neutral
                   // Cast visual effect: Circle_Small_Blue_Out
            },
        });
        ev_action.send(PlayerActionEvent(vec![action]));
    }

    if action_state.pressed(&PlayerAction::Skip) {
        let action = Box::new(SkipAction);
        ev_action.send(PlayerActionEvent(vec![action]));
    }
}

use bevy::prelude::*;

use crate::{
    graphics::{
        action_animation::ActionAnimationSet, anim_data::AnimKey, animations::Animator,
        get_world_position, pokemons::PokemonAnimationState, POKEMON_Z, POSITION_TOLERANCE,
        WALK_SPEED,
    },
    map::{GameMap, Position, TileType},
    pieces::Occupier,
    vector2_int::Vector2Int,
};

use super::{orient_entity, Action, ActionExecutedEvent, RunningAction};

pub struct WalkActionPlugin;

impl Plugin for WalkActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (walk_action_system).in_set(ActionAnimationSet::Prepare),
        );
    }
}

#[derive(Debug, Clone)]
pub struct WalkAction {
    pub entity: Entity,
    pub to: Vector2Int,
    pub from: Vector2Int,
}

#[derive(Debug, Component)]
pub struct WalkActionComponent {
    pub entity: Entity,
    pub to: Vec3,
    pub from: Vec3,
    t: f32,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        orient_entity(world, self.entity, self.to);

        // get the position of the entity
        let mut position = world.get_mut::<Position>(self.entity).ok_or(())?;
        position.0 = self.to;

        world.entity_mut(self.entity).insert(WalkActionComponent {
            entity: self.entity,
            from: get_world_position(&self.from, POKEMON_Z),
            to: get_world_position(&self.to, POKEMON_Z),
            t: 0.,
        });

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        true
    }

    fn can_execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<GameMap>() else {
            return false;
        };

        // check if the targeted position is on the board
        let Some(tile) = board.tiles.get(&self.to) else {
            return false;
        };

        if *tile != TileType::Ground {
            return false;
        }

        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.to)
        {
            return false;
        };

        true
    }
}

fn walk_action_system(
    mut query: Query<(
        &mut WalkActionComponent,
        &mut PokemonAnimationState,
        &mut Transform,
        &Animator,
    )>,
    mut ev_action_executed: EventWriter<ActionExecutedEvent>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut walk_action, mut animation_state, mut transform, animator) in query.iter_mut() {
        if walk_action.t == 0. {
            animation_state.0 = AnimKey::Walk;
        }

        let d = (walk_action.to - transform.translation).length();

        if d > POSITION_TOLERANCE {
            //ev_animation_playing.send(ActionAnimationPlayingEvent);
            walk_action.t = (walk_action.t + WALK_SPEED * time.delta_seconds()).clamp(0., 1.);
            transform.translation = walk_action.from.lerp(walk_action.to, walk_action.t);
            continue;
        }

        // the entity is at the desired path position
        // transform.translation = walk_action.to;

        // if !animator.is_finished() {
        //     continue;
        // }

        commands
            .entity(walk_action.entity)
            .remove::<(RunningAction, WalkActionComponent)>();

        ev_action_executed.send(ActionExecutedEvent(walk_action.entity));
    }
}

use crate::{
    graphics::{
        action_animation::ActionAnimationSet, anim_data::AnimKey, pokemons::PokemonAnimationState,
    },
    map::Position,
    pieces::{FacingOrientation, Orientation},
    vector2_int::Vector2Int,
    GamePlayingSet,
};
use bevy::prelude::*;
use dyn_clone::DynClone;
use std::{any::Any, fmt::Debug};

pub use self::action_queue::*;
mod action_queue;
pub mod damage_action;
pub mod destroy_wall_action;
pub mod melee_hit_action;
pub mod projectile_action;
pub mod skip_action;
pub mod spell_action;
pub mod walk_action;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionExecutedEvent>()
            .add_event::<ActionQueueProcessedEvent>()
            .add_event::<ProcessingActionEvent>()
            .init_resource::<ActionQueue>()
            .add_systems(
                Update,
                (process_action_queue).in_set(GamePlayingSet::Actions),
            );
        // .add_systems(
        //     Update,
        //     handle_action_finished.in_set(ActionAnimationSet::Flush),
        // );
    }
}

dyn_clone::clone_trait_object!(Action);
pub trait Action: Send + Sync + DynClone + Debug {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn can_execute(&self, world: &mut World) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn is_parallel_execution(&self) -> bool;
}

/// Current executed action attached to an entity
#[derive(Component)]
pub struct RunningAction(pub Box<dyn Action>);

#[derive(Component, Default, Clone)]
pub struct NextActions(pub Vec<Box<dyn Action>>);

#[derive(Event, Debug)]
pub struct ProcessingActionEvent;

#[derive(Event, Debug)]
pub struct ActionExecutedEvent(Entity);

#[derive(Event, Debug)]
pub struct ActionQueueProcessedEvent;

pub fn orient_entity(world: &mut World, entity: Entity, target: Vector2Int) {
    let Some(grid_position) = world.get::<Position>(entity) else {
        return;
    };
    let direction = target - grid_position.0;

    let Some(mut facing_orientation) = world.get_mut::<FacingOrientation>(entity) else {
        return;
    };

    facing_orientation.0 = Orientation::from_vector(direction);
}

// fn handle_action_finished(
//     mut ev_animation_finished: EventReader<ActionExecutedEvent>,
//     mut query_animation_state: Query<&mut PokemonAnimationState>,
// ) {
//     for ev in ev_animation_finished.read() {
//         let Ok(mut animation_state) = query_animation_state.get_mut(ev.0) else {
//             continue;
//         };
//         animation_state.0 = AnimKey::Idle;
//     }
// }

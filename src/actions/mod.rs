use crate::{ivec2::OrientationExt, map::Position, pieces::FacingOrientation, GamePlayingSet};
use bevy::prelude::*;
use char_animation::orientation::Orientation;
use dyn_clone::DynClone;
use std::{any::Any, fmt::Debug};

pub use self::action_queue::*;
mod action_queue;
pub mod damage_action;
pub mod death_action;
pub mod destroy_wall_action;
pub mod melee_hit_action;
pub mod skip_action;
pub mod spell_action;
pub mod spell_hit_action;
pub mod spell_projectile_action;
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
pub struct ActionExecutedEvent {
    pub action: Box<dyn Action>,
    pub entity: Entity,
}

#[derive(Event, Debug)]
pub struct ActionQueueProcessedEvent;

pub fn orient_entity(world: &mut World, entity: Entity, target: IVec2) {
    let Some(grid_position) = world.get::<Position>(entity) else {
        return;
    };
    let direction = target - grid_position.0;

    let Some(mut facing_orientation) = world.get_mut::<FacingOrientation>(entity) else {
        return;
    };

    facing_orientation.0 = Orientation::from_vector(direction);
}

use bevy::prelude::*;

use super::Action;

pub struct SkipAction;

impl Action for SkipAction {
    fn execute(&self, _world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

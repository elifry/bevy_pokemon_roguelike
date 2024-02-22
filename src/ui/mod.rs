mod event_logger;

use bevy::prelude::*;

use crate::GameState;

use self::event_logger::{event_logger_ui, gather_logs, EventLogs};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventLogs>().add_systems(
            Update,
            (gather_logs, event_logger_ui).run_if(in_state(GameState::Playing)),
        );

        #[cfg(debug_assertions)]
        {
            // app.add_plugins(ResourceInspectorPlugin::<EventLogs>::default());
        }
    }
}

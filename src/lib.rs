use actions::ActionsPlugin;
use ai::AIPlugin;
use bevy::app::App;

use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use graphics::GraphicsPlugin;
use test::TestPlugin;
use visual_effects::VisualEffectsPlugin;

use map::MapPlugin;
use menu::MenuPlugin;
use pieces::PiecesPlugin;
use player::{PlayerActionEvent, PlayerPlugin};
use turn::TurnPlugin;

mod actions;
mod ai;
mod camera;
mod constants;
mod graphics;
mod map;
mod menu;
mod pieces;
mod player;
mod pokemons;
pub mod spells;
mod test;
mod turn;
pub mod utils;
mod vector2_int;
mod visual_effects;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    AssetsLoaded,
    // Spawn entities
    Initializing,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GamePlayingSet {
    Inputs,
    Controls,
    AI,
    TurnLogics,
    Animations,
    Actions,
    LateLogics,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .configure_sets(
                Update,
                (
                    GamePlayingSet::Inputs,
                    GamePlayingSet::Controls,
                    GamePlayingSet::AI.run_if(on_event::<PlayerActionEvent>()),
                    GamePlayingSet::TurnLogics,
                    GamePlayingSet::Animations,
                    GamePlayingSet::Actions,
                    GamePlayingSet::LateLogics,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_plugins((
                // LoadingPlugin, // custom assets loading system can't use for now
                MenuPlugin,
                MapPlugin,
                PiecesPlugin,
                GraphicsPlugin,
                CameraPlugin,
                PlayerPlugin,
                AIPlugin,
                ActionsPlugin,
                TurnPlugin,
                VisualEffectsPlugin,
                //Only for testing purposes
                TestPlugin,
            ));

        #[cfg(debug_assertions)]
        {
            app
                //.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
                .add_plugins(WorldInspectorPlugin::new());
        }
    }
}

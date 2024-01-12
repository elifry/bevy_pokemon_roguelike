use actions::ActionsPlugin;
use ai::AIPlugin;
use bevy::app::App;

use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use game_control::GameControlPlugin;
use graphics::GraphicsPlugin;

use map::MapPlugin;
use menu::MenuPlugin;
use pieces::PiecesPlugin;
use player::PlayerPlugin;
use turn::TurnPlugin;

mod actions;
mod ai;
mod camera;
mod game_control;
mod graphics;
mod loading;
mod map;
mod menu;
mod pieces;
mod player;
mod pokemons;
mod turn;
mod vector2_int;

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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            // LoadingPlugin, // custom assets loading system can't use for now
            MenuPlugin,
            MapPlugin,
            PiecesPlugin,
            GraphicsPlugin,
            CameraPlugin,
            PlayerPlugin,
            AIPlugin,
            GameControlPlugin,
            ActionsPlugin,
            TurnPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app
                //.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
                .add_plugins(WorldInspectorPlugin::new());
        }
    }
}

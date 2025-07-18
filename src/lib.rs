use actions::ActionsPlugin;
use ai::AIPlugin;
use bevy::app::App;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::{EguiPlugin, EguiSet, EguiSettings, EguiUserTextures};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bitmap_font::BitmapFontPlugin;
use camera::CameraPlugin;
use char_animation::CharAnimationPlugin;
use data::DataPlugin;
use graphics::GraphicsPlugin;
use loading::LoadingPlugin;
use pokemon_data::PokemonDataPlugin;
use pokemons::PokemonsPlugin;
use stats::StatsPlugin;
use test::TestPlugin;
use ui::UIPlugin;
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
mod data;
mod faction;
mod graphics;
mod ivec2;
pub mod loading;
mod map;
mod menu;
mod move_type;
mod pieces;
mod player;
mod pokemons;
pub mod spells;
mod stats;
mod test;
mod turn;
mod ui;
pub mod utils;
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
        app.init_state::<GameState>()
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
                BitmapFontPlugin,
                CharAnimationPlugin,
                PokemonDataPlugin,
                EguiPlugin,
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
            ))
            .add_plugins((
                StatsPlugin,
                DataPlugin,
                LoadingPlugin,
                PokemonsPlugin,
                UIPlugin,
            ))
            .add_systems(Update, update_ui_scale.run_if(in_state(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app
                //.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
                .add_plugins(WorldInspectorPlugin::new());
        }
    }
}

fn update_ui_scale(
    mut egui_query: Query<(&mut EguiSettings, Option<&PrimaryWindow>), With<Window>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    projection: Query<&OrthographicProjection, With<Camera>>,
) {
    if let Ok(window) = windows.get_single() {
        if let Ok(projection) = projection.get_single() {
            // Find the primary window's egui settings
            for (mut egui_settings, primary_window) in egui_query.iter_mut() {
                if primary_window.is_some() {
                    match projection.scaling_mode {
                        bevy::render::camera::ScalingMode::FixedVertical(fixed_ratio) => {
                            let window_height = window.height();
                            let scale = window_height / fixed_ratio / (projection.scale);
                            egui_settings.scale_factor = scale;
                        }
                        bevy::render::camera::ScalingMode::FixedHorizontal(fixed_ratio) => {
                            let window_width = window.width();
                            let scale = window_width / fixed_ratio / (projection.scale);
                            egui_settings.scale_factor = scale;
                        }
                        _ => {}
                    }
                    break; // Only update the primary window
                }
            }
        }
    }
}

use std::any::Any;

use bevy::prelude::*;

use crate::{map::Position, pieces::Actor, player::Player, GameState};

pub struct GameControlPlugin;

impl Plugin for GameControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameControlEvent>().add_systems(
            Update,
            player_input_controls.run_if(in_state(GameState::Playing)),
        );
    }
}

const DIR_KEY_MAPPING: [(KeyCode, IVec2); 4] = [
    (KeyCode::W, IVec2 { x: 0, y: 1 }),
    (KeyCode::S, IVec2 { x: 0, y: -1 }),
    (KeyCode::A, IVec2 { x: -1, y: 0 }),
    (KeyCode::D, IVec2 { x: 1, y: 0 }),
];

#[derive(Event)]
pub struct GameControlEvent(pub GameControl);

pub enum GameControl {
    Target(IVec2),
    Other,
}

fn player_input_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Position), With<Player>>,
    mut ev_game_control: EventWriter<GameControlEvent>,
) {
    let Ok(position) = player_query.get_single_mut() else {
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if !keyboard_input.just_pressed(key) {
            continue;
        }
        ev_game_control.send(GameControlEvent(GameControl::Target(position.0 + dir)));
    }
}

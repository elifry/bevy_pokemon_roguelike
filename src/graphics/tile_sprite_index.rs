use std::collections::HashMap;

use bevy::prelude::*;

use crate::{map::TileType, vector2_int::Vector2Int};

const ROW: usize = 21;

const X: i8 = 0; // Tile should be not present
const O: i8 = 1; // Tile should be present
const U: i8 = 2; // Whatever

static PATTERNS: &[([[i8; 3]; 3], usize); 47] = &[
    // ROW 0
    ([[U, X, U], [X, O, O], [U, O, O]], 0),
    ([[U, X, U], [O, O, O], [O, O, O]], 1),
    ([[U, X, U], [O, O, X], [O, O, U]], 2),
    // ROW 1
    ([[U, O, O], [X, O, O], [U, O, O]], ROW),
    ([[O, O, O], [O, O, O], [O, O, O]], 1 + ROW),
    ([[O, O, U], [O, O, X], [O, O, U]], 2 + ROW),
    // ROW 2
    ([[U, O, O], [X, O, O], [U, X, U]], ROW * 2),
    ([[O, O, O], [O, O, O], [U, X, U]], 1 + ROW * 2),
    ([[O, O, U], [O, O, X], [U, X, U]], 2 + ROW * 2),
    // ROW 3
    ([[X, X, U], [X, O, O], [U, O, X]], ROW * 3),
    ([[U, X, U], [O, O, O], [U, X, U]], 1 + ROW * 3),
    ([[U, X, X], [O, O, X], [X, O, U]], 2 + ROW * 3),
    // ROW 4
    ([[U, O, U], [X, O, X], [U, O, U]], ROW * 4),
    ([[U, X, U], [X, O, X], [U, X, U]], 1 + ROW * 4),
    // ROW 5
    ([[U, O, X], [X, O, O], [X, X, U]], ROW * 5),
    ([[X, O, U], [O, O, X], [U, X, X]], 2 + ROW * 5),
    // ROW 6
    ([[X, X, X], [X, O, X], [U, O, U]], 1 + ROW * 6),
    // ROW 7
    ([[X, X, U], [X, O, O], [X, X, U]], ROW * 7),
    ([[X, O, X], [O, O, O], [X, O, X]], 1 + ROW * 7),
    ([[U, X, X], [O, O, X], [U, X, X]], 2 + ROW * 7),
    // ROW 8
    ([[U, O, U], [X, O, X], [X, X, X]], 1 + ROW * 8),
    // ROW 9
    ([[X, X, X], [O, O, O], [X, O, X]], 1 + ROW * 9),
    // ROW 10
    ([[X, O, U], [X, O, O], [X, O, X]], ROW * 10),
    ([[U, O, X], [O, O, X], [X, O, X]], 2 + ROW * 10),
    // ROW 11
    ([[X, O, X], [O, O, O], [X, X, X]], 1 + ROW * 11),
    // ROW 12
    ([[O, O, O], [O, O, O], [X, O, X]], 1 + ROW * 12),
    // ROW 13
    ([[O, O, X], [O, O, O], [O, O, X]], ROW * 13),
    ([[X, O, O], [O, O, O], [X, O, O]], 2 + ROW * 13),
    // ROW 14
    ([[X, O, X], [O, O, O], [O, O, O]], 1 + ROW * 14),
    // ROW 15
    ([[O, O, O], [O, O, O], [O, O, X]], ROW * 15),
    ([[O, O, O], [O, O, O], [X, O, O]], 1 + ROW * 15),
    // ROW 16
    ([[O, O, X], [O, O, O], [O, O, O]], ROW * 16),
    ([[X, O, O], [O, O, O], [O, O, O]], 1 + ROW * 16),
    // ROW 17
    ([[U, O, O], [X, O, O], [U, O, X]], ROW * 17),
    ([[O, O, U], [O, O, X], [X, O, U]], 1 + ROW * 17),
    // ROW 18
    ([[U, O, X], [X, O, O], [U, O, O]], ROW * 18),
    ([[X, O, U], [O, O, X], [O, O, U]], 1 + ROW * 18),
    // ROW 19
    ([[U, X, X], [O, O, O], [O, O, X]], ROW * 19),
    ([[X, X, U], [O, O, O], [X, O, O]], 1 + ROW * 19),
    // ROW 20
    ([[O, O, X], [O, O, O], [U, X, X]], ROW * 20),
    ([[X, O, O], [O, O, O], [X, X, U]], 1 + ROW * 20),
    // ROW 21
    ([[X, O, X], [O, O, O], [X, O, O]], ROW * 21),
    ([[X, O, X], [O, O, O], [O, O, X]], 1 + ROW * 21),
    // ROW 22
    ([[X, O, O], [O, O, O], [X, O, X]], ROW * 22),
    ([[O, O, X], [O, O, O], [X, O, X]], 1 + ROW * 22),
    // ROW 23
    ([[X, O, O], [O, O, O], [O, O, X]], ROW * 23),
    ([[O, O, X], [O, O, O], [X, O, O]], 1 + ROW * 23),
];

/// Check if the position match any pattern in PATTERNS (first tuple element) then returns the associated index (present in the second of the tuple).
/// Pattern encoding, 0 = no element or tile_type different from the tested one, 1 = same tile type
pub fn find_sprite_index_tile(position: &Vector2Int, map: &HashMap<Vector2Int, TileType>) -> usize {
    let tile_type = *map.get(position).unwrap();

    for (pattern, index) in PATTERNS {
        let mut pattern_match = true;
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, &value) in row.iter().enumerate() {
                let neighbor_position = Vector2Int {
                    x: position.x + dx as i32 - 1,
                    y: position.y - dy as i32 + 1, // axis is inverted on bevy
                };

                let neighbor_type = map.get(&neighbor_position);

                if value != U && (value == O && neighbor_type != Some(&tile_type))
                    || (value == X && neighbor_type == Some(&tile_type))
                {
                    pattern_match = false;
                    break;
                }
            }
            if !pattern_match {
                break;
            }
        }

        if pattern_match {
            return *index;
        }
    }

    warn!(
        "Unable to find tile index for {:?} {:?}",
        position, tile_type
    );

    #[cfg(debug_assertions)]
    for dy in 0..=2 {
        for dx in 0..=2 {
            let neighbor_position = Vector2Int {
                x: position.x + dx - 1,
                y: position.y + dy - 1,
            };

            let neighbor_type = map.get(&neighbor_position);

            // X: No neighbor
            if neighbor_type.is_none() || neighbor_type != Some(&tile_type) {
                print!("X ");
                continue;
            }

            print!("O ");
        }
        println!();
    }

    // If no pattern matches, return a default index or handle as needed
    4 + ROW * 4 // for example
}

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use bevy::{log::warn, math::IVec2};
use char_animation::orientation::Orientation;

pub fn find_path(
    start: IVec2,
    end: IVec2,
    tiles: &HashSet<IVec2>,
    blockers: &HashSet<IVec2>,
) -> Option<VecDeque<IVec2>> {
    let mut queue = BinaryHeap::new();
    queue.push(Node { v: start, cost: 0 });
    let mut visited = HashMap::new();
    visited.insert(start, 0);
    let mut came_from = HashMap::new();

    while let Some(Node { v, cost }) = queue.pop() {
        if v == end {
            break;
        }
        for dir in ORTHO_DIRECTIONS {
            let n = v + dir;
            let new_cost = cost + 1;
            if !tiles.contains(&n) {
                continue;
            }
            // we allow the target to be a blocker
            if blockers.contains(&n) && n != end {
                continue;
            }
            match visited.get(&n) {
                Some(c) if *c <= new_cost => (),
                _ => {
                    visited.insert(n, new_cost);
                    queue.push(Node {
                        v: n,
                        cost: new_cost,
                    });
                    came_from.insert(n, v);
                }
            }
        }
    }
    let mut path = VecDeque::new();
    let mut cur = end;
    while let Some(v) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *v;
        if cur == start {
            return Some(path);
        }
    }
    None
}

// helper struct for the path finder
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pub v: IVec2,
    pub cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.v.cmp(&other.v))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait IVec2Ext {
    fn magnitude(&self) -> f64;
    fn normalize(&self) -> IVec2;
    fn eq(&self, other: &Self) -> bool;
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
    fn cmp(&self, other: &Self) -> Ordering;
    fn manhattan(&self, other: IVec2) -> i32;

    const UP: IVec2 = IVec2 { x: 0, y: 1 };
    const DOWN: IVec2 = IVec2 { x: 0, y: -1 };
    const LEFT: IVec2 = IVec2 { x: -1, y: 0 };
    const RIGHT: IVec2 = IVec2 { x: 1, y: 0 };
}

impl IVec2Ext for IVec2 {
    fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    fn normalize(&self) -> IVec2 {
        let mag = self.magnitude();
        if mag != 0.0 {
            IVec2 {
                x: (self.x as f64 / mag) as i32,
                y: (self.y as f64 / mag) as i32,
            }
        } else {
            // Handle the case when the magnitude is 0 to avoid division by zero
            IVec2 { x: 0, y: 0 }
        }
    }

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x {
            self.y.partial_cmp(&other.y)
        } else {
            self.x.partial_cmp(&other.x)
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }

    fn manhattan(&self, other: IVec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub trait OrientationExt {
    fn from_vector(direction: IVec2) -> Orientation;
    fn to_vector(&self) -> IVec2;
}

impl OrientationExt for Orientation {
    fn from_vector(direction: IVec2) -> Self {
        match direction.normalize() {
            IVec2 { x: 0, y: -1 } => Orientation::South,
            IVec2 { x: 1, y: -1 } => Orientation::SouthEst,
            IVec2 { x: 1, y: 0 } => Orientation::Est,
            IVec2 { x: 1, y: 1 } => Orientation::NorthEst,
            IVec2 { x: 0, y: 1 } => Orientation::North,
            IVec2 { x: -1, y: 1 } => Orientation::NorthWest,
            IVec2 { x: -1, y: 0 } => Orientation::West,
            IVec2 { x: -1, y: -1 } => Orientation::SouthWest,
            IVec2 { x: _, y: _ } => {
                warn!("unable to get orientation from {:?}", direction);
                Orientation::South
            }
        }
    }

    fn to_vector(&self) -> IVec2 {
        match self {
            Orientation::South => IVec2 { x: 0, y: -1 },
            Orientation::SouthEst => IVec2 { x: 1, y: -1 },
            Orientation::Est => IVec2 { x: 1, y: 0 },
            Orientation::NorthEst => IVec2 { x: 1, y: 1 },
            Orientation::North => IVec2 { x: 0, y: 1 },
            Orientation::NorthWest => IVec2 { x: -1, y: 1 },
            Orientation::West => IVec2 { x: -1, y: 0 },
            Orientation::SouthWest => IVec2 { x: -1, y: -1 },
        }
    }
}

pub const ORTHO_DIRECTIONS: [IVec2; 4] = [IVec2::UP, IVec2::DOWN, IVec2::LEFT, IVec2::RIGHT];

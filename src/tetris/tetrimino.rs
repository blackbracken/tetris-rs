use std::collections::HashMap;

use MinoRotation::*;
use Tetrimino::*;

use crate::tetris::game::{MinoBlock, Point};

type MinoShape = Vec<Vec<bool>>;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tetrimino {
    T,
    S,
    Z,
    L,
    J,
    O,
    I,
}

impl Tetrimino {
    pub fn center(&self) -> Point {
        match self {
            T | S | Z | L | J | I => (1, 1).into(),
            O => (0, 0).into(),
        }
    }

    pub fn edge_length(&self) -> usize {
        match self {
            T | S | Z | L | J => 3,
            O => 2,
            I => 4,
        }
    }

    pub fn shapes(&self) -> HashMap<MinoRotation, MinoShape> {
        match self {
            T => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [0, 1, 0],
                        [1, 1, 1],
                        [0, 0, 0],
                ),
                Clockwise90 => rect_vec!(
                        [0, 1, 0],
                        [0, 1, 1],
                        [0, 1, 0],
                ),
                Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [1, 1, 1],
                        [0, 1, 0],
                ),
                Clockwise270 => rect_vec!(
                        [0, 1, 0],
                        [1, 1, 0],
                        [0, 1, 0],
                ),
            },
            S => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [0, 1, 1],
                        [1, 1, 0],
                        [0, 0, 0],
                ),
                Clockwise90 => rect_vec!(
                        [0, 1, 0],
                        [0, 1, 1],
                        [0, 0, 1],
                ),
                Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [0, 1, 1],
                        [1, 1, 0],
                ),
                Clockwise270 => rect_vec!(
                        [1, 0, 0],
                        [1, 1, 0],
                        [0, 1, 0],
                ),
            },
            Z => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [1, 1, 0],
                        [0, 1, 1],
                        [0, 0, 0],
                ),
                Clockwise90 => rect_vec!(
                        [0, 0, 1],
                        [0, 1, 1],
                        [0, 1, 0],
                ),
                Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [1, 1, 0],
                        [0, 1, 1],
                ),
                Clockwise270 => rect_vec!(
                        [0, 1, 0],
                        [1, 1, 0],
                        [1, 0, 0],
                ),
            },
            L => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [0, 0, 1],
                        [1, 1, 1],
                        [0, 0, 0],
                ),
                Clockwise90 => rect_vec!(
                        [0, 1, 0],
                        [0, 1, 0],
                        [0, 1, 1],
                ),
                Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [1, 1, 1],
                        [1, 0, 0],
                ),
                Clockwise270 => rect_vec!(
                        [1, 1, 0],
                        [0, 1, 0],
                        [0, 1, 0],
                ),
            },
            J => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [1, 0, 0],
                        [1, 1, 1],
                        [0, 0, 0],
                ),
                Clockwise90 => rect_vec!(
                        [0, 1, 1],
                        [0, 1, 0],
                        [0, 1, 0],
                ),
                Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [1, 1, 1],
                        [0, 0, 1],
                ),
                Clockwise270 => rect_vec!(
                        [0, 1, 0],
                        [0, 1, 0],
                        [1, 1, 0],
                ),
            },
            O => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [1, 1],
                        [1, 1],
                ),
                Clockwise90 => rect_vec!(
                        [1, 1],
                        [1, 1],
                ),
                Clockwise180 => rect_vec!(
                        [1, 1],
                        [1, 1],
                ),
                Clockwise270 => rect_vec!(
                        [1, 1],
                        [1, 1],
                ),
            },
            I => maplit::hashmap! {
                Clockwise => rect_vec!(
                        [0, 0, 0, 0],
                        [1, 1, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                ),
                Clockwise90 => rect_vec!(
                        [0, 0, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 1, 0],
                ),
                Clockwise180 => rect_vec!(
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                        [1, 1, 1, 1],
                        [0, 0, 0, 0],
                ),
                Clockwise270 => rect_vec!(
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                )
            },
        }
    }

    pub fn wall_kicks(&self) -> HashMap<MinoRotation, [Point; 5]> {
        match self {
            T | S | Z | L | J => maplit::hashmap! {
                Clockwise => [
                        (0, 0).into(),
                        (-1, 0).into(),
                        (-1, -1).into(),
                        (0, 2).into(),
                        (-1, 2).into(),
                ],
                Clockwise90 => [
                        (0, 0).into(),
                        (1, 0).into(),
                        (1, 1).into(),
                        (0, -2).into(),
                        (1, -2).into(),
                ],
                Clockwise180 => [
                        (0, 0).into(),
                        (1, 0).into(),
                        (1, -1).into(),
                        (0, 2).into(),
                        (1, 2).into(),
                ],
                Clockwise270 => [
                        (0, 0).into(),
                        (-1, 0).into(),
                        (-1, 1).into(),
                        (0, -2).into(),
                        (-1, -2).into(),
                ],
            },
            O => maplit::hashmap! {
                Clockwise => [(0, 0).into(); 5],
                Clockwise90 => [(0, 0).into(); 5],
                Clockwise180 => [(0, 0).into(); 5],
                Clockwise270 => [(0, 0).into(); 5],
            },
            I => maplit::hashmap! {
                Clockwise => [
                        (0, 0).into(),
                        (-2, 0).into(),
                        (1, 0).into(),
                        (-2, 1).into(),
                        (1, -2).into(),
                ],
                Clockwise90 => [
                        (0, 0).into(),
                        (-1, 0).into(),
                        (2, 0).into(),
                        (-1, -2).into(),
                        (2, 1).into(),
                ],
                Clockwise180 => [
                        (0, 0).into(),
                        (2, 0).into(),
                        (-1, 0).into(),
                        (2, -1).into(),
                        (-1, -2).into(),
                ],
                Clockwise270 => [
                        (0, 0).into(),
                        (1, 0).into(),
                        (-2, 0).into(),
                        (1, 2).into(),
                        (-2, -1).into(),
                ],
            },
        }
    }

    pub fn block(&self) -> MinoBlock {
        use MinoBlock::*;

        match self {
            T => PURPLE,
            S => GREEN,
            Z => RED,
            L => ORANGE,
            J => BLUE,
            O => YELLOW,
            I => AQUA,
        }
    }

    pub fn all() -> Vec<Tetrimino> {
        vec!(T, S, Z, L, J, O, I)
    }
}

// clockwise angles starts at 12 o'clock position
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MinoRotation {
    Clockwise,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

impl MinoRotation {
    pub fn left(&self) -> MinoRotation {
        match self {
            Clockwise => Clockwise270,
            Clockwise90 => Clockwise,
            Clockwise180 => Clockwise90,
            Clockwise270 => Clockwise180,
        }
    }

    pub fn right(&self) -> MinoRotation {
        match self {
            Clockwise => Clockwise90,
            Clockwise90 => Clockwise180,
            Clockwise180 => Clockwise270,
            Clockwise270 => Clockwise,
        }
    }
}

impl Default for MinoRotation {
    fn default() -> Self {
        Clockwise
    }
}

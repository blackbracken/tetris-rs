use std::collections::HashMap;

use crate::tetris::game::{MinoBlock, Point};

type MinoShape = Vec<Vec<bool>>;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tetrimino {
    T,
}

impl Tetrimino {
    pub fn center(&self) -> Point {
        match self {
            Tetrimino::T => (1, 1).into()
        }
    }

    pub fn edge_length(&self) -> usize {
        match self {
            Tetrimino::T => 3,
        }
    }

    pub fn shapes(&self) -> HashMap<MinoRotation, MinoShape> {
        match self {
            Tetrimino::T => maplit::hashmap! {
                MinoRotation::Clockwise => rect_vec!(
                        [0, 1, 0],
                        [1, 1, 1],
                        [0, 0, 0],
                ),
                MinoRotation::Clockwise90 => rect_vec!(
                        [0, 1, 0],
                        [0, 1, 1],
                        [0, 1, 0],
                ),
                MinoRotation::Clockwise180 => rect_vec!(
                        [0, 0, 0],
                        [1, 1, 1],
                        [0, 1, 0],
                ),
                MinoRotation::Clockwise270 => rect_vec!(
                        [0, 1, 0],
                        [1, 1, 0],
                        [0, 1, 0],
                ),
            }
        }
    }

    pub fn spin_offsets(&self) -> HashMap<MinoRotation, Vec<Point>> {
        match self {
            Tetrimino::T => maplit::hashmap! {
                MinoRotation::Clockwise => vec!(
                        (0, 0).into(),
                        (-1, 0).into(),
                        (-1, -1).into(),
                        (0, 2).into(),
                        (-1, 2).into(),
                ),
                MinoRotation::Clockwise90 => vec!(
                        (0, 0).into(),
                        (1, 0).into(),
                        (1, 1).into(),
                        (0, -2).into(),
                        (1, -2).into(),
                ),
                MinoRotation::Clockwise180 => vec!(
                        (0, 0).into(),
                        (1, 0).into(),
                        (1, -1).into(),
                        (0, 2).into(),
                        (1, 2).into(),
                ),
                MinoRotation::Clockwise270 => vec!(
                        (0, 0).into(),
                        (-1, 0).into(),
                        (-1, 1).into(),
                        (0, -2).into(),
                        (-1, -2).into(),
                ),
            }
        }
    }

    pub fn block(&self) -> MinoBlock {
        match self {
            Tetrimino::T => MinoBlock::PURPLE
        }
    }

    pub fn all() -> Vec<Tetrimino> {
        use crate::tetris::tetrimino::Tetrimino::T;

        vec!(T)
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
            MinoRotation::Clockwise => MinoRotation::Clockwise270,
            MinoRotation::Clockwise90 => MinoRotation::Clockwise,
            MinoRotation::Clockwise180 => MinoRotation::Clockwise90,
            MinoRotation::Clockwise270 => MinoRotation::Clockwise180,
        }
    }

    pub fn right(&self) -> MinoRotation {
        match self {
            MinoRotation::Clockwise => MinoRotation::Clockwise90,
            MinoRotation::Clockwise90 => MinoRotation::Clockwise180,
            MinoRotation::Clockwise180 => MinoRotation::Clockwise270,
            MinoRotation::Clockwise270 => MinoRotation::Clockwise,
        }
    }
}

impl Default for MinoRotation {
    fn default() -> Self {
        MinoRotation::Clockwise
    }
}

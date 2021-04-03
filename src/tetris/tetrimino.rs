use core::ops;
use std::collections::HashMap;

use MinoRotation::*;
use Tetrimino::*;

use crate::macros::rect_vec;
use crate::tetris::game::{MinoBlock, Point, SpinDirection};

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

    // The field on tetris-rs is positive as Y increases downward, so Y needs to be multiplied by -1.
    pub fn wall_kick_offsets(&self, from: &MinoRotation, direction: &SpinDirection) -> [WallKickOffset; 5] {
        let to = from.spin(direction);

        match self {
            // ref. https://tetrisch.github.io/main/srs.html
            T | S | Z | L | J => {
                let _0: WallKickOffset = (0, 0).into();

                let _1: WallKickOffset = match to {
                    Clockwise90 => (-1, 0),
                    Clockwise270 => (1, 0),
                    Clockwise | Clockwise180 => match direction {
                        SpinDirection::Right => (-1, 0),
                        SpinDirection::Left => (1, 0),
                    }
                }.into();

                let _2: WallKickOffset = match to {
                    Clockwise90 | Clockwise270 => (0, -1),
                    Clockwise | Clockwise180 => (0, 1),
                }.into();
                let _2 = _2 + &_1;

                let _3: WallKickOffset = match to {
                    Clockwise90 | Clockwise270 => (0, 2),
                    Clockwise | Clockwise180 => (0, -2),
                }.into();

                let _4: WallKickOffset = match to {
                    Clockwise90 => (-1, 0),
                    Clockwise270 => (1, 0),
                    Clockwise | Clockwise180 => match direction {
                        SpinDirection::Right => (-1, 0),
                        SpinDirection::Left => (1, 0),
                    },
                }.into();
                let _4 = _4 + &_3;

                [_0, _1, _2, _3, _4]
            }

            O => [(0, 0).into(); 5],

            // ref. https://tetris.fandom.com/wiki/SRS/
            I => {
                match to {
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
                }
            }
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

#[derive(Copy, Clone)]
pub struct WallKickOffset {
    pub x: isize,
    pub y: isize,
}

impl WallKickOffset {
    fn new(x: isize, y: isize) -> WallKickOffset {
        WallKickOffset { x, y }
    }
}

impl Into<WallKickOffset> for (isize, isize) {
    fn into(self) -> WallKickOffset {
        WallKickOffset::new(self.0, self.1)
    }
}

impl ops::Add<&WallKickOffset> for WallKickOffset {
    type Output = WallKickOffset;

    fn add(self, rhs: &WallKickOffset) -> Self::Output {
        WallKickOffset::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// clockwise angles starts at 12 o'clock position
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MinoRotation {
    Clockwise = 0,
    Clockwise90 = 90,
    Clockwise180 = 180,
    Clockwise270 = 270,
}

// TODO: calc numerically
impl MinoRotation {
    pub fn spin(&self, direction: &SpinDirection) -> MinoRotation {
        match direction {
            SpinDirection::Left => self.left(),
            SpinDirection::Right => self.right(),
        }
    }

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

    fn inverse(&self) -> MinoRotation {
        match self {
            Clockwise => Clockwise180,
            Clockwise90 => Clockwise270,
            Clockwise180 => Clockwise,
            Clockwise270 => Clockwise90,
        }
    }
}

impl Default for MinoRotation {
    fn default() -> Self {
        Clockwise
    }
}

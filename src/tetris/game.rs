use std::collections::VecDeque;

use rand::prelude::SliceRandom;

use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

pub const GAME_FIELD_WIDTH: usize = 10;
pub const GAME_FIELD_HEIGHT: usize = 22;

pub type Field = [[MinoBlock; GAME_FIELD_WIDTH]; GAME_FIELD_HEIGHT];

pub struct Game {
    pub board: Board,
    bag: MinoBag,
}

impl Game {
    pub fn new() -> Game {
        let mut bag = MinoBag::new();
        let dropping = bag.pop();

        Game {
            board: Board::new(dropping),
            bag,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    confirmed_field: Field,
    dropping: Tetrimino,
    dropping_point: Point,
    dropping_rotation: MinoRotation,
}

impl Board {
    fn new(dropping: Tetrimino) -> Board {
        Board {
            confirmed_field: [[MinoBlock::AIR; GAME_FIELD_WIDTH]; GAME_FIELD_HEIGHT],
            dropping,
            dropping_point: (4, (GAME_FIELD_HEIGHT - 20) as isize).into(),
            dropping_rotation: MinoRotation::Clockwise,
        }
    }

    pub fn field(&self) -> Field {
        let mut field = self.confirmed_field.clone();
        let shapes = self.dropping.shapes();
        let shape = shapes.get(&self.dropping_rotation).unwrap();

        let center = &self.dropping.center();
        let dropping_at = &self.dropping_point;

        for (block_y, line) in shape.iter().enumerate() {
            for (block_x, &exists) in line.iter().enumerate() {
                let x = (dropping_at.x + (block_x as isize) - center.x) as usize;
                let y = (dropping_at.y + (block_y as isize) - center.y) as usize;

                if exists {
                    field[y][x] = self.dropping.block();
                }
            }
        }

        field
    }

    pub fn try_move_left(&mut self) -> bool {
        self.try_move_x(-1)
    }

    pub fn try_move_right(&mut self) -> bool {
        self.try_move_x(1)
    }

    fn try_move_x(&mut self, addition: isize) -> bool {
        let clone = &mut self.clone();

        let manipulation = |board: &mut Board| {
            board.dropping_point.x += addition;
        };

        manipulation(clone);

        if clone.establishes_field() {
            manipulation(self);
        }

        clone.establishes_field()
    }

    pub fn try_spin_left(&mut self) -> bool {
        self.try_spin(RotateDirection::Left)
    }

    pub fn try_spin_right(&mut self) -> bool {
        self.try_spin(RotateDirection::Right)
    }

    fn try_spin(&mut self, direction: RotateDirection) -> bool {
        let rotation = match direction {
            RotateDirection::Left => self.dropping_rotation.left(),
            RotateDirection::Right => self.dropping_rotation.right(),
        };
        let offsets = self.dropping.spin_offsets();
        let offsets = offsets.get(&rotation).unwrap();

        let manipulation = |board: &mut Board, offset: &Point| {
            board.dropping_rotation = rotation;
            board.dropping_point.x += offset.x;
            board.dropping_point.y += offset.y;
        };

        offsets.into_iter()
            .find(|&offset| {
                let clone = &mut self.clone();
                manipulation(clone, offset);

                clone.establishes_field()
            })
            .map(|offset| manipulation(self, offset))
            .is_some()
    }

    fn establishes_field(&self) -> bool {
        self.calc_dropping_mino_points().iter()
            .all(|&point| {
                if !(0..(GAME_FIELD_HEIGHT as isize)).contains(&point.y)
                    || !(0..(GAME_FIELD_WIDTH as isize)).contains(&point.x) {
                    return false;
                }

                !self.confirmed_field[point.y as usize][point.x as usize].exists()
            })
    }

    fn calc_dropping_mino_points(&self) -> Vec<Point> {
        let shapes = self.dropping.shapes();
        let shape = shapes.get(&self.dropping_rotation).unwrap();

        let center = &self.dropping.center();
        let dropping_at = &self.dropping_point;

        shape.iter()
            .enumerate()
            .flat_map(|(mass_y, line)| {
                line.iter()
                    .enumerate()
                    .flat_map(|(mass_x, &exists)| {
                        let x = dropping_at.x + (mass_x as isize) - center.x;
                        let y = dropping_at.y + (mass_y as isize) - center.y;

                        Some((x, y).into()).filter(|_| exists)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

struct MinoBag {
    queue: VecDeque<Tetrimino>
}

impl MinoBag {
    fn new() -> MinoBag {
        let mut queue = MinoBag::gen_shuffled_all_minos();
        let mut added = MinoBag::gen_shuffled_all_minos();
        queue.append(&mut added);

        MinoBag {
            queue: queue.into(),
        }
    }

    pub fn pop(&mut self) -> Tetrimino {
        let p = self.queue.pop_front().unwrap();

        if self.queue.len() < Tetrimino::all().len() {
            let mut added = MinoBag::gen_shuffled_all_minos();
            self.queue.append(&mut added.into());
        }

        p
    }

    pub fn peek(&self, amount: usize) -> Vec<Tetrimino> {
        if amount > Tetrimino::all().len() {
            panic!("the amount of minos must be equal to or lower than the amount of tetrimino types");
        }

        (0..amount)
            .map(|idx| self.queue.get(idx).unwrap().clone())
            .collect::<Vec<_>>()
    }


    fn gen_shuffled_all_minos() -> Vec<Tetrimino> {
        let mut rng = rand::thread_rng();

        let mut s = Tetrimino::all();
        s.shuffle(&mut rng);

        s
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum MinoBlock {
    AQUA,
    YELLOW,
    PURPLE,
    BLUE,
    ORANGE,
    GREEN,
    RED,
    AIR,
}

impl MinoBlock {
    fn exists(&self) -> bool {
        match self {
            MinoBlock::AIR => false,
            _ => true,
        }
    }
}

enum Movement {
    MoveLeft,
    MoveRight,
    DropSoftly,
    DropHardly,
    SpinLeft,
    SpinRight,
}

enum RotateDirection {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Point { x: isize, y: isize }

impl Into<Point> for (isize, isize) {
    fn into(self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type ExistenceField = [[bool; GAME_FIELD_WIDTH]; GAME_FIELD_HEIGHT];

    trait LikeExistenceField {
        fn to_existences(&self) -> ExistenceField;

        fn similar<F: LikeExistenceField>(&self, to: F) -> bool {
            self.to_existences() == to.to_existences()
        }
    }

    impl LikeExistenceField for Field {
        fn to_existences(&self) -> ExistenceField {
            let mut f = ExistenceField::default();
            self.iter().enumerate().for_each(|(y, line)| {
                line.iter().enumerate().for_each(|(x, block)| {
                    f[y][x] = block.exists();
                })
            });

            f
        }
    }

    impl LikeExistenceField for [[i32; GAME_FIELD_WIDTH]; GAME_FIELD_HEIGHT] {
        fn to_existences(&self) -> ExistenceField {
            let mut f = ExistenceField::default();
            self.iter().enumerate().for_each(|(y, line)| {
                line.iter().enumerate().for_each(|(x, &n)| {
                    f[y][x] = n > 0;
                })
            });

            f
        }
    }

    #[test]
    fn rect_vec_returns_2d_vec() {
        let rect_vec: Vec<Vec<bool>> = rect_vec!(
        [0, 0, 0, 0],
        [1, 0, 1, 0],
        [0, 1, 0, 1],
        [1, 1, 1, 1],
    );

        assert_eq!(
            rect_vec,
            vec!(
                vec!(false, false, false, false),
                vec!(true, false, true, false),
                vec!(false, true, false, true),
                vec!(true, true, true, true),
            )
        );
    }

    #[test]
    fn field_is_only_with_dropping_on_init() {
        let board = Board::new(Tetrimino::T);

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_right_once() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_right());

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_right_to_limit() {
        let mut board = Board::new(Tetrimino::T);
        for _ in 0..4 {
            assert!(board.try_move_right());
        }
        assert!(!board.try_move_right());

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_left_once() {
        let mut board = Board::new(Tetrimino::T);
        assert!(board.try_move_left());

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }

    #[test]
    fn move_left_to_limit() {
        let mut board = Board::new(Tetrimino::T);
        for _ in 0..3 {
            assert!(board.try_move_left());
        }
        assert!(!board.try_move_left());

        let expected = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        assert!(board.field().similar(expected));
    }
}